use serde::Serialize;
use tide::Response;
use tide::http::mime;

pub trait HandlebarsExt {
    fn render_response<T>(&self, template: &'static str, data: &T) -> Response
    where 
        T: Serialize;
}

impl HandlebarsExt for handlebars::Handlebars<'static> {
    fn render_response<T>(&self, template: &'static str, data: &T) -> Response 
    where
        T: Serialize
    {
        Response::builder(200)
            .body(self.render(template, data).unwrap())
            .content_type(mime::HTML)
            .build()
    } 
} 
