use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use wireframe::{ParseStatus, Parser, ParserConfig};

use crate::response::Response;
use crate::router::{RequestContext, Router};

pub struct Server {
    address: String,
    router: Arc<Router>,
    config: ParserConfig,
}

impl Server {
    pub fn new(address: &str, router: Router) -> Self {
        Self {
            address: address.to_string(),
            router: Arc::new(router),
            config: ParserConfig::default(),
        }
    }

    pub fn with_config(mut self, config: ParserConfig) -> Self {
        self.config = config;
        self
    }

    pub async fn run(self) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.address).await?;
        log::info!("Server started on {}", self.address);

        loop {
            let (mut socket, peer_addr) = listener.accept().await?;
            log::debug!("Accepted connection from {}", peer_addr);

            let router = Arc::clone(&self.router);
            let config = self.config.clone();

            tokio::spawn(async move {
                let mut parser = Parser::with_config(config);
                let mut buf = vec![0u8; 4096];

                loop {
                    match socket.read(&mut buf).await {
                        Ok(0) => break, // Connection closed
                        Ok(n) => {
                            match parser.feed(&buf[..n]) {
                                Ok(ParseStatus::Complete(_)) => {
                                    if let Ok(request) = parser.finish() {
                                        let response = if let Some(handler) =
                                            router.route(&request.method, &request.uri)
                                        {
                                            handler.handle(RequestContext { req: request }).await
                                        } else {
                                            Response::not_found()
                                        };

                                        if let Err(e) =
                                            Self::write_response(&mut socket, response).await
                                        {
                                            log::error!("Error writing response: {}", e);
                                        }
                                    } else {
                                        let _ = Self::write_response(
                                            &mut socket,
                                            Response::bad_request(),
                                        )
                                        .await;
                                    }
                                    break;
                                }
                                Ok(ParseStatus::Incomplete) => continue, // Need more data
                                Err(_) => {
                                    let _ =
                                        Self::write_response(&mut socket, Response::bad_request())
                                            .await;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            log::error!("Error reading from socket: {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }

    async fn write_response(
        socket: &mut tokio::net::TcpStream,
        response: Response,
    ) -> std::io::Result<()> {
        let mut head = format!("HTTP/1.1 {} {}\r\n", response.status, response.status_text);

        for (k, v) in &response.headers {
            head.push_str(&format!("{}: {}\r\n", k, v));
        }

        // Ensure connection close for this simple implementation
        if !response.headers.contains_key("Connection") {
            head.push_str("Connection: close\r\n");
        }

        head.push_str("\r\n");

        socket.write_all(head.as_bytes()).await?;
        if !response.body.is_empty() {
            socket.write_all(&response.body).await?;
        }
        Ok(())
    }
}
