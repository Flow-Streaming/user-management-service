use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use tracing::{error, info};

use crate::models::{AppState, AuthResponse, SignUpRequest, SignUpResponse};

pub async fn sign_up_user(
    State(config): State<AppState>,
    Json(payload): Json<SignUpRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!("Received sign-up request for email: {}", payload.email);

    let client = Client::new();

    // Perform basic validation
    if payload.email.is_empty() || payload.password.is_empty() {
        error!("Sign-up failed: Empty email or password");
        return Err((
            StatusCode::BAD_REQUEST,
            "Email and password are required".to_string(),
        ));
    }

    // Prepare authentication data
    let auth_data = json!({
        "email": payload.email,
        "password": payload.password,
        "data": {
            "username": payload.username
        }
    });

    // Step 1: Sign up with Supabase Auth
    let auth_response = match client
        .post(format!("{}/auth/v1/signup", config.supabase_url))
        .header("apikey", &config.supabase_api_key)
        .header("Content-Type", "application/json")
        .json(&auth_data)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to send sign-up request: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send sign-up request".to_string(),
            ));
        }
    };

    // Check if the auth request was successful
    if !auth_response.status().is_success() {
        let error_text = auth_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("Auth signup failed: {}", error_text);
        return Err((StatusCode::BAD_REQUEST, error_text));
    }

    // Parse the auth response
    let auth_response_body = match auth_response.json::<AuthResponse>().await {
        Ok(body) => body,
        Err(e) => {
            error!("Failed to parse auth response: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    let user_id = auth_response_body.user.id;

    let user_data = json!({
        "id": user_id,
        "email": payload.email,
        "username": payload.username,
        "subscription_plan": "basic",
        "profile_picture_url": match payload.profile_picture_url {
            Some(url) => url,
            None => "default_profile_picture_url".to_string(),
        },
        "last_login": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap()
    });

    // Create user in the database
    let db_response = match client
        .post(format!("{}/rest/v1/users", config.supabase_url))
        .header("apikey", &config.supabase_api_key)
        .header(
            "Authorization",
            format!("Bearer {}", &config.supabase_api_key),
        )
        .header("Content-Type", "application/json")
        .header("Prefer", "return=minimal")
        .json(&user_data)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to create user in database: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    // Check database response
    if !db_response.status().is_success() {
        let error_text = db_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("Database user creation failed: {}", error_text);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, error_text));
    }

    info!("User successfully created with ID: {}", user_id);
    Ok((
        StatusCode::CREATED,
        Json(SignUpResponse {
            user_id,
            access_token: auth_response_body.access_token,
        }),
    ))
}

pub async fn get_user_data(
    State(config): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let client = Client::new();

    // Fetch user data from the database
    let db_response = match client
        .get(format!(
            "{}/rest/v1/users?id=eq.{}",
            config.supabase_url, user_id
        ))
        .header("apikey", &config.supabase_api_key)
        .header(
            "Authorization",
            format!("Bearer {}", &config.supabase_api_key),
        )
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to fetch user data from database: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    // Check database response
    if !db_response.status().is_success() {
        let error_text = db_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("Database user data fetch failed: {}", error_text);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, error_text));
    }

    let user_data = db_response.json::<Vec<Value>>().await.unwrap_or_default();

    // Return user data
    Ok(Json(user_data))
}

pub async fn update_user_data(
    State(config): State<AppState>,
    Path(user_id): Path<String>,
    Json(user_data): Json<Value>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let client = Client::new();

    // Update user data in the database
    let db_response = match client
        .patch(format!(
            "{}/rest/v1/users?id=eq.{}",
            config.supabase_url, user_id
        ))
        .header("apikey", &config.supabase_api_key)
        .header(
            "Authorization",
            format!("Bearer {}", &config.supabase_api_key),
        )
        .json(&user_data)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            error!("Failed to update user data in database: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    };

    // Check database response
    if !db_response.status().is_success() {
        let error_text = db_response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        error!("Database user data update failed: {}", error_text);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, error_text));
    }

    // Return success response
    Ok(StatusCode::OK)
}
