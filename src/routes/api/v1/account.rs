use serde::{Deserialize, Serialize};
use tide::{Result, prelude::json, Response};

use crate::{routes::Route, core::{state::ApplicationState, accounts, models::account::Account, sessions}};

pub struct AccountAPI;

impl Route for AccountAPI {
    fn register(app: &mut tide::Server<ApplicationState>) {
        app.at("/login").post(AccountAPI::request_login);
        app.at("/signup").post(AccountAPI::request_register);
    }
}

impl AccountAPI {
    async fn request_login(mut req: tide::Request<ApplicationState>) -> Result {
        let account = match req.body_json::<LoginInfo>().await {
            Ok(account) => account,
            Err(_) => {
                return Ok(Response::builder(403)
                    .body("Failed to parse login info.")
                    .build());
            }
        };

        let mut success = false;
        let mut error = String::new();

        let pass_hash = accounts::hash_password(account.password);

        match accounts::login(account.email, pass_hash).await {
            Ok(uid) => {
                success = true;
                let session_id = req.session().id().to_string();
                if let Err(err) = sessions::create(session_id, uid.clone()).await {
                    log::error!("Failed to create session for UID {uid}: {err}");
                }
                else {
                    log::debug!("Created session for UID {uid}");
                }
            },
            Err(err) => error = err,
        };
        
        Ok(
            json!({
                "success": success,
                "error": error,
            }).into()
        )
    }

    async fn request_register(mut req: tide::Request<ApplicationState>) -> Result {
        let info = match req.body_json::<RegisterInfo>().await {
            Ok(info) => info,
            Err(_) => {
                return Ok(Response::builder(403)
                        .body("Failed to parse registration info.")
                        .build());
            }
        };

        let mut success = false;
        let mut error = String::new();

        let user = Account::partial(info.firstname, info.surname, info.email, info.password);

        match accounts::register(user, true, true).await {
            Ok(_) => success = true,
            Err(err) => error = err,
        };

        Ok(
            json!({
                "success": success,
                "error": error
            }).into()
        )
    }
}

#[derive(Deserialize, Serialize, Clone)]
struct LoginInfo {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Clone)]
struct RegisterInfo {
    pub email: String,
    pub firstname: String,
    pub surname: String,
    pub password: String
}