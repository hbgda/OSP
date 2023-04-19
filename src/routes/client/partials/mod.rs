use crate::core::state::ApplicationState;

use crate::Route;
use super::get_template_path;

pub struct Partials;

impl Route for Partials {
    fn register(app: &mut tide::Server<ApplicationState>) {
        let mut hb = app.state().hb.lock().unwrap();
        
        log::info!("| - Layouts");
        Partials::register_layouts(&mut hb);
        // log::info!("| - Components");
        // Partials::register_components(&mut hb); 
    }    
}

impl Partials {
    fn register_components(hb: &mut handlebars::Handlebars) {
        todo!()
    }

    fn register_layouts(hb: &mut handlebars::Handlebars) {
        hb.register_template_file("base_layout", get_template_path("/pages/layouts/base_layout.hbs")).unwrap();
        hb.register_template_file("main_layout", get_template_path("/pages/layouts/main_layout.hbs")).unwrap();
    }
}
