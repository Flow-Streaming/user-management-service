use dotenv::dotenv;

use crate::models::AppState;

pub const SUBSCRIPTION_PLAN_BASIC: &str = "basic";
pub const SUBSCRIPTION_PLAN_STANDARD: &str = "standard";
pub const SUBSCRIPTION_PLAN_PREMIUM: &str = "premium";

pub fn load_config() -> AppState {
    // Load environment variables
    dotenv().ok();

    // Initialize application state
    AppState {
        supabase_url: std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        supabase_api_key: std::env::var("SUPABASE_API_KEY").expect("SUPABASE_API_KEY must be set"),
    }
}
