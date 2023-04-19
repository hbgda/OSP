use chrono::{Duration, Utc};
use once_cell::sync::Lazy;

use super::{database, models::session::Session};

/// If a sessions lifetime exceeds this duration it will be deleted.
const SESSION_DURATION: Lazy<Duration> = Lazy::new(|| Duration::days(180));

/// Attempts to find an existing session.
/// Will also ensure the sessions validity.
/// 
/// # Arguments
/// * `session_id` - [`String`] containing the session id to search for.
pub async fn get(session_id: String) -> Option<Session> {
    let db = database::get();

    let filter = db.filter()
        // Session matching session_id
        .eq("session_id", session_id.clone().into())
        .build();

    match db.get("sessions", filter).await {
        Ok(session) => {
            // Parse session
            let session = serde_json::from_value::<Session>(session).unwrap();
            if !is_valid(session.clone()) {
                // Delete if session is no longer valid
                let _ = delete(session_id.clone()).await;
                return None;
            }

            Some(session)
        },
        Err(err) => {
            log::debug!("Failed to query database for session. {err}");
            None
        }
    }
}

/// Attempts to create a new user session.
/// 
/// # Arguments
/// * `session_id` - [`String`] containing a users session identifier.
/// * `uid` - [`String`] containing the users unique identifier. 
/// 
/// # Examples
/// ```
/// use crate::core::sessions;
/// 
/// async fn api_login(req: Request) -> ApiResponse {
///     let uid = /* ... */;
/// 
///     let session_id = req.session().id().to_string();
///     if let Err(err) = sessions::create(session_id, uid).await {
///         log::error!("Failed to create user session. {err}");
///     }
/// 
///     /* ... */
/// }
/// ```
pub async fn create(session_id: String, uid: String) -> Result<Session, String> {
    // Check if session exists
    if let Some(_) = get(session_id.clone()).await {
        log::warn!("Attempted to create session with existing id {session_id}");
        return Err("Session already exists.".into());
    }

    let db = database::get();
    let session = Session {
        session_id,
        uid,
        created: Utc::now().timestamp_millis(),
        valid: true
    };

    // Convert to raw json
    let session_raw = serde_json::to_value(session.clone()).unwrap();
    if let Err(err) = db.insert("sessions", &session_raw).await {
        log::error!("Failed to persist session {:?}: {err}", session);
        return Err("Failed to create session.".into());
    }

    Ok(session)
}

pub fn is_valid(session: Session) -> bool {
    // If the session has already been invalidated just return false.
    if !session.valid {
        return false;
    }

    // Check that the difference between Utc now and session.created is within the bounds of SESSION_DURATION.
    Utc::now().timestamp_millis() - session.created < SESSION_DURATION.num_milliseconds()
}

/// Attempts to delete the session matching `session_id`.
/// 
/// # Arguments
/// * `session_id` - [`String`] containing the session id.
pub async fn delete(session_id: String) -> Result<(), String> {
    let db = database::get();

    let filter = db.filter()
        .eq("session_id", session_id.clone().into())
        .build();

    if let Err(err) = db.delete("sessions", filter).await {
        log::error!("Failed to delete session {session_id}: {err}");
        return Err(err);
    }

    Ok(())
}