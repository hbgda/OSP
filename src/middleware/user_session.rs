use tide::{Middleware, Request, Next, Result};

use crate::core::sessions;
use super::MiddlewareData;

pub struct UserSessionMiddleware;

impl UserSessionMiddleware {
    pub fn new() -> UserSessionMiddleware {
        UserSessionMiddleware { }
    }
}

#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for UserSessionMiddleware {
    async fn handle(&self, mut req: Request<State>, next: Next<'_, State>) -> Result {
        let tide_session = req.session().id().to_string();
        if let Some(session) = sessions::get(tide_session.clone()).await {
            log::debug!("Session exists {tide_session}" );

            // Retrieve existing middleware data if present, otherwise create new.
            let mut ext = match req.ext::<MiddlewareData>() {
                Some(ext) => ext.clone(),
                None => MiddlewareData::new()
            };
            ext["uid"] = session.uid.into();

            // Update request
            req.set_ext(ext);
        }

        Ok(next.run(req).await)
    }
}