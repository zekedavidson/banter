/// Application configuration loaded from environment variables.

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_service_role_key: String,
    pub supabase_jwt_secret: String,
    pub database_url: String,
    pub livekit_url: String,
    pub livekit_api_key: String,
    pub livekit_api_secret: String,
    pub backend_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            supabase_url: env("SUPABASE_URL"),
            supabase_anon_key: env("SUPABASE_ANON_KEY"),
            supabase_service_role_key: env("SUPABASE_SERVICE_ROLE_KEY"),
            supabase_jwt_secret: env("SUPABASE_JWT_SECRET"),
            database_url: env("DATABASE_URL"),
            livekit_url: env("LIVEKIT_URL"),
            livekit_api_key: env("LIVEKIT_API_KEY"),
            livekit_api_secret: env("LIVEKIT_API_SECRET"),
            backend_port: env("BACKEND_PORT")
                .parse()
                .unwrap_or(8080),
        }
    }
}

fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Missing environment variable: {key}"))
}
