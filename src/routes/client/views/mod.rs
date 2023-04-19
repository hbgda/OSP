pub mod home;
pub mod login;
pub mod error;
pub mod signup;

use crate::core::state::ApplicationState;
use crate::Route;

pub struct Views;

impl Route for Views {
    fn register(app: &mut tide::Server<ApplicationState>) {
        home::HomeView::register(app);    
        login::LoginView::register(app);
        signup::SignupView::register(app);
        error::ErrorView::register(app);
    }
}
