use tide::prelude::json;

use crate::core::ext::handlebars::HandlebarsExt;

use crate::core::state::ApplicationState;
use crate::routes::client::get_template_path;
use super::Route;

pub struct ErrorView;

impl Route for ErrorView {
    fn register(app: &mut tide::Server<ApplicationState>) {
        log::info!("| - Error");
        app.state().hb.lock().unwrap().register_template_file("error", get_template_path("/pages/error.hbs")).unwrap();

        app.at("/*").get(|req: tide::Request<ApplicationState>| async move {
            let hb = req.state().hb.lock().unwrap();    
            Ok(hb.render_response("error", &json!({"title": "Error", "parent": "main_layout"})))
        }); 
    }    
}
