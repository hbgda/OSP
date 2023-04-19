use super::{models::account::Account, database, validation::{validate_name, validate_email, validate_password}};
use base64ct::{Base64, Encoding};
use sha2::{Sha256, Digest};

/// Attempts to find an account.
/// 
/// Returns the `uid` if successful.
/// 
/// # Arguments
/// 
/// * `email` - [`String`] containing a valid email address.
/// 
/// # Examples
/// 
/// ```
/// use crate::core::accounts;
/// 
/// let user = Account {
///     // ...
/// }
/// let uid = accounts::register(user, true).await?;
/// let found = accounts::exists(user.email).await;
/// 
/// assert_eq!(Some(uid), found)
/// ```
pub async fn exists(email: String) -> Option<String> {
    let db = database::get();

    let filter = db.filter()
        // Account with matching email
        .eq("email", email.into())
        .build();

    // Request entry with a matching email and password
    let result = db.get("accounts", filter).await;

    // Return Some(uid) if result is Ok, else None
    if let Ok(account) = result {
        return Some(account["uid"].as_str().unwrap().to_string());
    }
    None
}

/// Checks that the provided `pass_hash` matches that of the [`Account`] corresponding to `uid`
/// 
/// # Arguments
/// 
/// * `uid` - [`String`] containing the users unique identifier.
/// * `pass_hash` - [`String`] containing the hash to compare against.
/// 
/// # Examples 
/// 
/// ```
/// use crate::core::accounts;
/// 
/// 
/// 
/// ```
pub async fn verify_pass_hash(uid: String, pass_hash: String) -> bool {
    let db = database::get();

    let filter = db.filter()
        .eq("uid", uid.into())
        .eq("pass_hash", pass_hash.into())
        .build();

    let result = db.find("accounts", filter).await;

    match result {
        Ok(_) => true,
        Err(_) => false
    }
}

/// Attempts to verify the provided login details.
/// 
/// Return the accounts `uid` if successful.
/// 
/// # Arguments
/// 
/// * `email` - [`String`] containing a valid email address.
/// * `pass_hash` - [`String`] containg the [`Base64`] encoded [`Sha256`] hash of the users password
/// 
/// [`Sha256`]: https://en.wikipedia.org/wiki/SHA-2
/// [`Base64`]: https://en.wikipedia.org/wiki/Base64
/// 
/// # Examples 
/// 
/// ```
/// use crate::core::accounts;
/// 
/// async fn api_login(req: Request) -> ApiResponse {
///     let { email, pass_hash } = req.args;
///     
///     match accounts::login(email, pass_hash).await {
///         Some(session) => {
///             return ApiResponse {
///                 status: 200,
///                 body: json!({
///                     "session": session
///                 })
///             }
///         },
///         Err(err) => {
///             return ApiResponse {
///                 status: 403,
///                 body: json!({
///                     "error": err
///                 })
///             }
///         }
///     }
/// }
/// 
/// ```
pub async fn login(email: String, pass_hash: String) -> Result<String, String> {
    let account_id = exists(email).await;
    if let None = account_id {
        return Err("No account found with that email.".into())
    }

    let uid = account_id.unwrap();

    log::debug!("Login | UID: {uid} | Pass: {pass_hash}");

    if !verify_pass_hash(uid.clone(), pass_hash).await {
        return Err("Incorrect password.".into())
    }

    Ok(uid)
}

/// Create a new user account.
/// Checks that the provided details are valid.
/// 
/// Returns the users `uid` if successful.
/// 
/// # Arguments
/// 
/// * `user` - [`Account`] containing the users provided information.
/// * `generate_id` - [`bool`], if true the `uid` in `user` will be overwritten with one provided by [accounts::uuid](super::accounts::uuid)
/// * `should_hash` - [`bool`], if true `user.pass_hash` will be run through [accounts::hash_password][super::accounts::hash_password]. Should only be true if `user.pass_hash` is plaintext.
/// 
/// # Examples 
/// 
/// ```
/// use crate::core::accounts;
/// 
/// async fn api_register(req: Request) -> ApiResponse {
///     let account = req.body::<Account>()?;
///     
///     match accounts.register(account, true, true).await {
///         Some(user_id) => {
///             return ApiResponse {
///                 status: 200,
///                 body: json!({
///                     "user_id": user_id
///                 })
///             }
///         },
///         Err(err) => {
///             return ApiResponse {
///                 status: 400,
///                 body: json!({
///                     "error": err
///                 })
///             }
///         }
///     }
/// }
/// 
/// ```
pub async fn register(mut user: Account, generate_id: bool, should_hash: bool) -> Result<String, String> {   
    
    if !validate_name(user.firstname.clone()) 
    || !validate_name(user.surname.clone())
    || !validate_email(user.email.clone())
    || (!should_hash && !validate_password(user.pass_hash.clone())) {
        return Err("Invalid registration details. Please try again.".into())
    }

    let db = database::get();

    // Ensure account does not already exist
    if let Some(_) = exists(user.email.clone()).await {
        return Err("Account already exists!".into())
    }

    if generate_id {
        user.uid = uuid();
    }

    if should_hash {
        user.pass_hash = hash_password(user.pass_hash)
    }

    // TODO: Validate account info

    // Create a database entry for the new account.
    let result = db.insert(
        "accounts",
        &serde_json::to_value(user.clone()).unwrap()
    ).await;
    
    if let Err(_) = result {
        return Err("Failed to register account, try again later.".into())
    }

    Ok(user.uid)
}

/// Generates a unique user ID using [`UUIDv4`]
/// 
/// [`UUIDv4`]: https://en.wikipedia.org/wiki/Universally_unique_identifier#Version_4_(random)
pub fn uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

/// Generates a secure password [`Base64`] encoded [`Sha256`] hash.
/// 
/// [`Sha256`]: https://en.wikipedia.org/wiki/SHA-2
/// [`Base64`]: https://en.wikipedia.org/wiki/Base64
/// 
/// # Arguments
/// 
/// * `password` - [`String`] containing the plaintext password.
///
/// # Examples 
/// 
/// ```
/// use crate::core::accounts;
/// 
/// let password = "superSecurePassword007".to_string();
/// let hash = accounts::hash_password(password);
/// 
/// assert_eq!(
///     "NzMwNWU3ZjJmOWY1NmVlNjA2ZTMzZWU3ZjJhNTQ5Y2RiODM1YWUxMmRiNGY0NjM0ODdkNzVhNGZkM2EyNDc4MA==".to_string()
///     hash
/// )
/// 
/// 
/// ```
pub fn hash_password(password: String) -> String {
    // Empty Sha256 instance
    let mut password_hash = Sha256::new();
    // Use contents of `password` for hashing
    password_hash.update(password);
    // Finalize hashing and encode as base64
    Base64::encode_string(
        &password_hash.finalize()
    )
}