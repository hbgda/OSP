use std::sync::{Arc, Mutex};

use handlebars::Handlebars;

#[derive(Clone)]
pub struct ApplicationState {
    pub hb: Arc<Mutex<Handlebars<'static>>>
}
