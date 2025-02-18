pub mod schema;

use init::server_init::server_init_proc;
use mimalloc::MiMalloc;
use tracing::{info, level_filters};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

pub const MEANING_OF_LIFE: u128 = 22_398_254_448_911u128;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// modules tree
pub mod domain {
    pub mod user;
}
pub mod dto {
    pub mod common {}
    pub mod emails {
        pub mod verify_your_email;
    }
    pub mod requests {
        pub mod user {
            pub mod check_if_user_exists_request;
            pub mod login_request;
            pub mod reset_password_request;
            pub mod signup_request;
            pub mod verify_user_email_request;
        }
    }
    pub mod responses {
        pub mod user {
            pub mod email_validate_response;
            pub mod login_response;
            pub mod logout_response;
            pub mod reset_password_request_response;
            pub mod signup_response;
        }
        pub mod response_data;
        pub mod response_meta;
    }
}
pub mod errors {
    pub mod code_error;
}
pub mod handlers {
    pub mod user {
        pub mod check_if_user_exists;
        pub mod login;
        pub mod logout;
        pub mod reset_password_request;
        pub mod signup;
        pub mod verify_user_email;
    }
    pub mod fallback;
    pub mod root;
}
pub mod routers {
    pub mod middleware {
        pub mod auth;
        pub mod logging;
    }
    pub mod main_router;
}
pub mod init {
    pub mod compile_regex;
    pub mod config;
    pub mod server_init;
    pub mod state;
}
pub mod jobs {
    pub mod auth {
        pub mod invalidate_sessions;
        pub mod purge_nonverified_users;
    }
    pub mod job_funcs {
        pub mod every_hour;
        pub mod every_minute;
        pub mod every_second;
        pub mod init_scheduler;
    }
}
pub mod util {
    pub mod email {
        pub mod emails;
    }
    pub mod string {
        pub mod validations;
    }
    pub mod crypto {
        pub mod hash_pw;
        pub mod random_pw;
        pub mod verify_pw;
    }
    pub mod time {
        pub mod duration_formatter;
        pub mod now;
    }
}

// main function
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let start = tokio::time::Instant::now();

    if std::env::var("IS_AWS").is_err() {
        dotenvy::dotenv().map_err(|e| anyhow::anyhow!("Failed to load .env: {}", e))?;
    }

    let app_name_version = std::env::var("APP_NAME_VERSION")
        .map_err(|e| anyhow::anyhow!("Failed to get APP_NAME_VERSION: {}", e))?;

    let filename = app_name_version.to_string();

    let file_appender =
        tracing_appender::rolling::daily(format!("./log/{}", app_name_version), filename);
    let (non_blocking_file, _guard) = tracing_appender::non_blocking(file_appender);

    // Create a console layer
    let console_layer = tracing_subscriber::fmt::layer()
        // .json()
        .with_ansi(true)
        .with_target(true)
        .with_filter(level_filters::LevelFilter::INFO);

    // Create a file layer
    let file_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .json()
        .with_writer(non_blocking_file)
        .with_filter(level_filters::LevelFilter::DEBUG);

    // Build a subscriber that combines both layers
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("Initializing server...");
    let server_handle = tokio::spawn(async move { server_init_proc(start).await });

    server_handle.await??;

    Ok(())
}
