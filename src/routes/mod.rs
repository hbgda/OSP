use crate::core::state::ApplicationState;

pub mod client;
pub mod api;

// Basic trait to be implemented on all API and view routes,
// this allows me to call `register` on any struct representing a route.
pub trait Route {
    fn register(app: &mut tide::Server<ApplicationState>);
}