use compositor_pipeline::pipeline;
use compositor_render::event_loop::EventLoop;
use crossbeam_channel::RecvTimeoutError;
use log::{error, info};

use serde_json::json;
use signal_hook::{consts, iterator::Signals};
use std::{io::Cursor, net::SocketAddr, sync::Arc, thread, time::Duration};
use tiny_http::{Header, Response, StatusCode};

use crate::{
    api::{self, Api, Request, ResponseHandler},
    error::ApiError,
};

pub struct Server {
    server: tiny_http::Server,
    content_type_json: Header,
}

impl Server {
    pub fn new(port: u16) -> Arc<Self> {
        Self {
            server: tiny_http::Server::http(SocketAddr::from(([0, 0, 0, 0], port))).unwrap(),
            content_type_json: Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                .unwrap(),
        }
        .into()
    }

    pub fn run(self: Arc<Self>) {
        info!("Listening on port {}", self.server.server_addr());
        let (mut api, event_loop) = self.handle_init();
        thread::spawn(move || {
            for mut raw_request in self.server.incoming_requests() {
                let result = Self::handle_request_after_init(&mut api, &mut raw_request);
                match result {
                    Ok(ResponseHandler::Ok) => {
                        self.send_response(raw_request, api::Response::Ok {});
                    }
                    Ok(ResponseHandler::Response(response)) => {
                        self.send_response(raw_request, response);
                    }
                    Ok(ResponseHandler::DeferredResponse(response)) => {
                        let server = self.clone();
                        thread::spawn(move || {
                            let response = response.recv_timeout(Duration::from_secs(60));
                            match response {
                                Ok(Ok(response)) => {
                                    server.send_response(raw_request, response);
                                }
                                Ok(Err(err)) => {
                                    server.send_err_response(raw_request, err);
                                }
                                Err(RecvTimeoutError::Timeout) => {
                                    server.send_err_response(
                                        raw_request,
                                        ApiError {
                                            error_code: "QUERY_TIMEOUT",
                                            message: "query timed out".to_string(),
                                            http_status_code: StatusCode(408),
                                        },
                                    );
                                }
                                Err(RecvTimeoutError::Disconnected) => {
                                    server.send_err_response(
                                        raw_request,
                                        ApiError {
                                            error_code: "INTERNAL_SERVER_ERROR",
                                            message: "Internal Server Error".to_string(),
                                            http_status_code: StatusCode(500),
                                        },
                                    );
                                }
                            };
                        });
                    }
                    Err(err) => {
                        self.send_err_response(raw_request, err);
                    }
                }
            }
        });

        let event_loop_fallback = || {
            let mut signals = Signals::new([consts::SIGINT]).unwrap();
            signals.forever().next();
        };
        if let Err(err) = event_loop.run_with_fallback(event_loop_fallback) {
            error!("Event loop run failed: {err}")
        }
    }

    fn handle_init(&self) -> (Api, EventLoop) {
        for mut raw_request in self.server.incoming_requests() {
            let result = self
                .handle_request_before_init(&mut raw_request)
                .and_then(Api::new);
            match result {
                Ok(new_api) => {
                    self.send_response(raw_request, api::Response::Ok {});
                    return new_api;
                }
                Err(err) => {
                    self.send_err_response(raw_request, err);
                }
            }
        }
        panic!("Server shutdown unexpectedly.")
    }

    fn handle_request_after_init(
        api: &mut Api,
        raw_request: &mut tiny_http::Request,
    ) -> Result<ResponseHandler, ApiError> {
        let request = Server::parse_request(raw_request)?;
        api.handle_request(request)
    }

    fn handle_request_before_init(
        &self,
        raw_request: &mut tiny_http::Request,
    ) -> Result<pipeline::Options, ApiError> {
        let request = Server::parse_request(raw_request)?;
        match request {
            Request::Init(opts) => Ok(opts),
            _ => Err(ApiError {
                error_code: "COMPOSITOR_NOT_INITIALIZED",
                message: "Compositor was not initialized, send \"init\" request first.".to_string(),
                http_status_code: StatusCode(400),
            }),
        }
    }

    fn send_response(&self, raw_request: tiny_http::Request, response: api::Response) {
        let response_result = serde_json::to_string(&response)
            .map_err(Into::into)
            .and_then(|body| {
                raw_request.respond(Response::new(
                    StatusCode(200),
                    vec![self.content_type_json.clone()],
                    Cursor::new(&body),
                    Some(body.len()),
                    None,
                ))
            });
        if let Err(err) = response_result {
            error!("Failed to send response {}.", err);
        }
    }

    fn send_err_response(&self, raw_request: tiny_http::Request, err: ApiError) {
        let response_result = serde_json::to_string(&json!({
            "msg": err.message,
            "error_code": err.error_code,
        }))
        .map_err(Into::into)
        .and_then(|body| {
            raw_request.respond(Response::new(
                err.http_status_code,
                vec![self.content_type_json.clone()],
                Cursor::new(&body),
                Some(body.len()),
                None,
            ))
        });
        if let Err(err) = response_result {
            error!("Failed to send response {}.", err);
        }
    }

    fn parse_request(request: &mut tiny_http::Request) -> Result<Request, ApiError> {
        serde_json::from_reader::<_, Request>(request.as_reader()).map_err(|err| ApiError {
            error_code: "MALFORMED_REQUEST",
            message: format!("Received malformed request:\n{err}"),
            http_status_code: StatusCode(400),
        })
    }
}
