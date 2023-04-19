use tide::{prelude::json, Redirect};

use crate::{routes::{Route, client::get_template_path}, core::{state::ApplicationState, ext::handlebars::HandlebarsExt, sessions}};

pub struct LoginView;

impl Route for LoginView {
    fn register(app: &mut tide::Server<ApplicationState>) {
        log::info!("| - /login");
        app.state().hb.lock().unwrap().register_template_file("login", get_template_path("/pages/login.hbs")).unwrap();

        app.at("/login").get(|req: tide::Request<ApplicationState>| async move {
            if let Some(_) = sessions::get(req.session().id().to_string()).await {
                log::debug!("Existing session found, redirecting to home...");
                // Redirect to index if user has an existing session.
                return Ok(
                    Redirect::new("/").into()
                );
            }
            let hb = req.state().hb.lock().unwrap();
            Ok(hb.render_response("login", &json!({"title": "Login", "parent": "base_layout"})))
        });
    }
}