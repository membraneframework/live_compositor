use std::process::Command;

use anyhow::{anyhow, Result};
use crossbeam_channel::Sender;
use futures_util::{SinkExt, StreamExt};
use live_compositor::config::read_config;
use log::info;
use serde_json::json;
use tokio_tungstenite::tungstenite;

use crate::CompositorInstance;

const BUNNY_URL: &str =
    "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4";

#[test]
pub fn offline_processing() -> Result<()> {
    const OUTPUT_FILE: &str = "/tmp/offline_processing_output.mp4";
    let mut config = read_config();
    config.queue_options.ahead_of_time_processing = true;
    config.queue_options.never_drop_output_frames = true;
    let instance = CompositorInstance::start(Some(config));
    let (msg_sender, msg_receiver) = crossbeam_channel::unbounded();
    start_server_msg_listener(instance.api_port, msg_sender);

    instance.send_request(
        "input/input_1/register",
        json!({
            "type": "mp4",
            "url":  BUNNY_URL,
            "required": true
        }),
    )?;

    instance.send_request(
        "output/output_1/register",
        json!({
            "type": "mp4",
            "path": OUTPUT_FILE,
            "video": {
                "resolution": {
                    "width": 640,
                    "height": 320
                },
                "encoder": {
                    "type": "ffmpeg_h264",
                    "preset": "ultrafast",
                },
                "initial": {
                    "root": {
                       "type": "view",
                       "children": [{
                            "type": "rescaler",
                            "child": {
                                "type": "input_stream",
                                "input_id": "input_1"
                            }
                        }]
                    }
                },
                "send_eos_when": { "all_inputs": true }
            },
            "audio": {
                "encoder": {
                    "type": "aac",
                    "channels": "stereo"
                },
                "initial": {
                    "inputs": [{ "input_id": "input_1" }]
                },
                "send_eos_when": { "all_inputs": true }
            }
        }),
    )?;

    instance.send_request(
        "input/input_1/unregister",
        json!({
            "schedule_time_ms": 2000
        }),
    )?;
    instance.send_request(
        "output/output_1/unregister",
        json!({
            "schedule_time_ms": 2000
        }),
    )?;

    instance.send_request("start", json!({}))?;

    for msg in msg_receiver.iter() {
        if let tungstenite::Message::Text(msg) = msg {
            if msg.contains("\"type\":\"OUTPUT_EOS\",\"output_id\":\"output_1\"") {
                info!("breaking");
                break;
            }
        }
    }

    Command::new("ffprobe")
        .args(["-v", "error", OUTPUT_FILE])
        .output()
        .map_err(|e| anyhow!("Invalid mp4 file. FFprobe error: {}", e))?;

    Ok(())
}

pub fn start_server_msg_listener(port: u16, event_sender: Sender<tungstenite::Message>) {
    std::thread::Builder::new()
        .name("Websocket Thread".to_string())
        .spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async { server_msg_listener(port, event_sender).await });
        })
        .unwrap();
}

async fn server_msg_listener(port: u16, event_sender: Sender<tungstenite::Message>) {
    let url = format!("ws://127.0.0.1:{}/ws", port);

    let (ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .expect("Failed to connect");

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let (mut outgoing, mut incoming) = ws_stream.split();

    let sender_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if let tungstenite::Message::Close(None) = &msg {
                let _ = outgoing.send(msg).await;
                return;
            }
            match outgoing.send(msg).await {
                Ok(()) => (),
                Err(e) => {
                    println!("Send Loop: {:?}", e);
                    let _ = outgoing.send(tungstenite::Message::Close(None)).await;
                    return;
                }
            }
        }
    });

    let receiver_task = tokio::spawn(async move {
        while let Some(result) = incoming.next().await {
            match result {
                Ok(tungstenite::Message::Close(_)) => {
                    let _ = tx.send(tungstenite::Message::Close(None));
                    return;
                }
                Ok(tungstenite::Message::Ping(data)) => {
                    if tx.send(tungstenite::Message::Pong(data)).is_err() {
                        return;
                    }
                }
                Err(_) => {
                    let _ = tx.send(tungstenite::Message::Close(None));
                    return;
                }
                Ok(msg) => {
                    event_sender.send(msg).unwrap();
                }
            }
        }
    });

    sender_task.await.unwrap();
    receiver_task.await.unwrap();
}
