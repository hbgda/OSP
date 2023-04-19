use tide::prelude::json;

use crate::{routes::{Route, client::get_template_path}, core::{state::ApplicationState, ext::handlebars::HandlebarsExt}};

pub struct SignupView;

impl Route for SignupView {
    fn register(app: &mut tide::Server<ApplicationState>) {
        log::info!("| - /signup");
        app.state().hb.lock().unwrap().register_template_file("signup", get_template_path("/pages/signup.hbs")).unwrap();

        app.at("/signup").get(|req: tide::Request<ApplicationState>| async move {
            let hb = req.state().hb.lock().unwrap();
            Ok(hb.render_response("signup", & json!({"title": "Sign Up", "parent": "base_layout"})))
        });
    }
}