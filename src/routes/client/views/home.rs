use tide::prelude::json;

use crate::core::ext::tide_request::TideRequestExt;
use crate::{routes::client::get_template_path, core::state::ApplicationState};
use crate::core::ext::handlebars::HandlebarsExt;
use super::Route;

pub struct HomeView;

impl Route for HomeView {
    fn register(app: &mut tide::Server<ApplicationState>) {
        log::info!("| - /home");
        app.state().hb.lock().unwrap().register_template_file("home", get_template_path("/pages/home.hbs")).unwrap();

        app.at("/").get(|req: tide::Request<ApplicationState>| async move {
            let hb = req.state().hb.lock().unwrap();
            let data = req.make_data(json!({
                "title": "Home",
                "parent": "main_layout"
            }));
            Ok(hb.render_response("home", &data))
        });
    }
} 
