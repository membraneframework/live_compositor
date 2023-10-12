use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Result};
use compositor_chromium::cef;
use compositor_render::{EMBED_SOURCES_MESSAGE, UNEMBED_SOURCE_MESSAGE};
use log::error;

use crate::state::{FrameInfo, State};

pub struct RenderProcessHandler {
    state: Arc<Mutex<State>>,
}

impl cef::RenderProcessHandler for RenderProcessHandler {
    fn on_context_created(
        &mut self,
        _browser: &cef::Browser,
        _frame: &cef::Frame,
        context: &cef::V8Context,
    ) {
        let mut global = context.global().unwrap();
        let ctx_entered = context.enter().unwrap();

        context.eval(include_str!("render_frame.js")).unwrap();
        if let Err(err) = self.register_native_funcs(&mut global, &ctx_entered) {
            error!("Failed to register native functions for V8Context: {err}");
        }
    }

    fn on_process_message_received(
        &mut self,
        _browser: &cef::Browser,
        frame: &cef::Frame,
        _source_process: cef::ProcessId,
        message: &cef::ProcessMessage,
    ) -> bool {
        const IS_HANDLED: bool = true;
        let result = match message.name().as_str() {
            EMBED_SOURCES_MESSAGE => self.embed_sources(message, frame),
            UNEMBED_SOURCE_MESSAGE => self.unembed_source(message, frame),
            name => Err(anyhow!("Unknown message type: {name}")),
        };

        if let Err(err) = result {
            error!("Error occurred while processing IPC message: {err}");
        }

        IS_HANDLED
    }
}

impl RenderProcessHandler {
    pub fn new(state: Arc<Mutex<State>>) -> Self {
        Self { state }
    }

    fn embed_sources(&self, msg: &cef::ProcessMessage, surface: &cef::Frame) -> Result<()> {
        let ctx = surface.v8_context()?;
        let ctx_entered = ctx.enter()?;
        let mut global = ctx.global()?;

        for i in (0..msg.size()).step_by(4) {
            let Some(shmem_path) = msg.read_string(i) else {
                return Err(anyhow!("Failed to read shared memory path at {i}"));
            };
            let Some(source_idx) = msg.read_int(i + 1) else {
                return Err(anyhow!("Failed to read input source index at {}", i + 1));
            };
            let Some(width) = msg.read_int(i + 2) else {
                return Err(anyhow!(
                    "Failed to read width of input {} at {}",
                    source_idx,
                    i + 2
                ));
            };
            let Some(height) = msg.read_int(i + 3) else {
                return Err(anyhow!(
                    "Failed to read height of input {} at {}",
                    source_idx,
                    i + 3
                ));
            };

            if width == 0 && height == 0 {
                continue;
            }

            let frame_info = FrameInfo {
                source_idx: source_idx as usize,
                width: width as u32,
                height: height as u32,
                shmem_path: shmem_path.into(),
            };

            self.render_frame(frame_info, &mut global, &ctx_entered)?;
        }

        Ok(())
    }

    fn render_frame(
        &self,
        frame_info: FrameInfo,
        global: &mut cef::V8Global,
        ctx_entered: &cef::V8ContextEntered,
    ) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        let source = match state.source(&frame_info.shmem_path) {
            Some(source) => source,
            None => state.create_source(frame_info, ctx_entered)?,
        };

        global.call_method(
            "renderFrame",
            &[
                &source.source_id,
                &source.array_buffer,
                &source.width,
                &source.height,
            ],
            ctx_entered,
        )?;

        Ok(())
    }

    fn unembed_source(&self, msg: &cef::ProcessMessage, surface: &cef::Frame) -> Result<()> {
        let Some(shmem_path) = msg.read_string(0) else {
            return Err(anyhow!("Failed to read shared memory path"));
        };
        let Some(node_id) = msg.read_string(1) else {
            return Err(anyhow!("Failed to read shared memory node ID"));
        };
        let Some(source_idx) = msg.read_int(2) else {
            return Err(anyhow!("Failed to read shared memory source index"));
        };

        let mut state = self.state.lock().unwrap();
        let shmem_path = PathBuf::from(shmem_path);
        if let Ok(input_name) = state.input_name(source_idx as usize) {
            let ctx = surface.v8_context()?;
            let ctx_entered = ctx.enter()?;
            let mut global = ctx.global()?;

            global.delete(&input_name, &ctx_entered)?;
            state.remove_source(&shmem_path);
        }

        let mut response = cef::ProcessMessage::new(UNEMBED_SOURCE_MESSAGE);
        response.write_string(0, node_id);
        response.write_int(1, source_idx);
        surface.send_process_message(cef::ProcessId::Browser, response)?;

        Ok(())
    }

    pub fn register_native_funcs(
        &self,
        global: &mut cef::V8Global,
        ctx_entered: &cef::V8ContextEntered,
    ) -> Result<()> {
        let state = self.state.clone();
        let func = cef::V8Function::new("register_inputs", move |args| {
            let mut input_mappings = Vec::new();
            for arg in args {
                let cef::V8Value::String(element_id) = arg else {
                    return Err("Expected string value".into());
                };
                let element_id = element_id.get().unwrap().into();
                if input_mappings.contains(&element_id) {
                    return Err(format!(
                        "\"{element_id}\" already exists in the provided input mappings"
                    )
                    .into());
                }

                input_mappings.push(element_id);
            }

            let mut state = state.lock().unwrap();
            state.set_input_mappings(input_mappings);
            Ok(cef::V8Undefined::new().into())
        });

        global.set(
            "register_inputs",
            &cef::V8Value::from(func),
            cef::V8PropertyAttribute::None,
            ctx_entered,
        )?;

        Ok(())
    }
}
