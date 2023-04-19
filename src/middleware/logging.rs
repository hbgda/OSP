use std::time::Instant;
use chrono;
use tide::{Middleware, Request, Next, Result, StatusCode};

pub struct LoggingMiddleware;

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl LoggingMiddleware {
    async fn log<'a, State: Clone + Send + Sync + 'static>(&'a self, req: Request<State>, next: Next<'a, State>) -> Result {
        let req_url = req.url().path().to_string();
        let req_method = req.method().to_string();
        
        let start = Instant::now();
        let response = next.run(req).await;
        let duration = start.elapsed().as_secs_f32();
        let timestamp = chrono::Utc::now().format("%Y-%d-%m %H:%M:%S");

        let status = response.status();
        let escape = LoggingMiddleware::escape_for(status);

        println!("{timestamp} {escape} [{status}] - {req_method} \x1B[0m  {req_url} - {duration}s ");
        
        Ok(response)
    }       

    fn escape_for(status: StatusCode) -> &'static str {
        if status.is_success() {
            "\x1B[38;5;41m"
        } 
        else if status.is_client_error() || status.is_server_error() {
            "\x1B[38;5;196m"
        } 
        else if status.is_informational() {
            "\x1B[38;5;44m"
        }
        else if status.is_redirection() {
            "\x1B[38;5;154m"
        }
        else {
            ""
        }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LoggingMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        self.log(req, next).await
    }
}