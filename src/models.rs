use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub supabase_url: String,
    pub supabase_api_key: String,
}
#[derive(Deserialize)]
pub struct SignUpRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub subscription_plan: String,
    pub profile_picture_url: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct SignUpResponse {
    pub user_id: String,
    pub access_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub access_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u32,
    pub expires_at: u64,
    pub refresh_token: String,
    pub user: UserDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserDetails {
    pub id: String,
    pub aud: String,
    pub role: String,
    pub email: String,
    pub email_confirmed_at: String,
    pub phone: String,
    pub last_sign_in_at: String,
    pub app_metadata: AppMetadata,
    pub user_metadata: UserMetadata,
    pub identities: Vec<Identity>,
    pub created_at: String,
    pub updated_at: String,
    pub is_anonymous: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppMetadata {
    pub provider: String,
    pub providers: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserMetadata {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Identity {
    pub identity_id: String,
    pub id: String,
    pub user_id: String,
    pub identity_data: IdentityData,
    pub provider: String,
    pub last_sign_in_at: String,
    pub created_at: String,
    pub updated_at: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IdentityData {
    pub email: String,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub sub: String,
}
