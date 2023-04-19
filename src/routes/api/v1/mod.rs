pub mod account;

use crate::{routes::Route, core::state::ApplicationState};

pub struct ApiV1;

impl Route for ApiV1 {
    fn register(app: &mut tide::Server<ApplicationState>) {
        let mut api = tide::with_state(app.state().clone());
        account::AccountAPI::register(&mut api);
        app.at("/_api/v1").nest(api);
    }
}