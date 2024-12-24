use dotenvy::dotenv;
use log::info;
use once_cell::sync::OnceCell;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::{Pool, Postgres};
use std::{env, fs};
use std::time::Duration;
use lazy_static::lazy_static;

lazy_static! {
    static ref DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
}

pub async fn init_db() {
    dotenv().ok();

    // อ่านค่าตัวแปรจาก environment
    let db_user = env::var("DB_USER").expect("DB_USER is not set");
    let db_pass = env::var("DB_PASS").expect("DB_PASS is not set");
    let db_host = env::var("DB_HOST").expect("DB_HOST is not set");
    let db_port = env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let db_name = env::var("DB_NAME").expect("DB_NAME is not set");

    // SSL/TLS (optional)
    let ssl_mode = match env::var("DB_SSLMODE")
        .unwrap_or_else(|_| "disable".to_string()).as_str() {
        "disable" => PgSslMode::Disable,
        "prefer" => PgSslMode::Prefer,
        "require" => PgSslMode::Require,
        "verify-ca" => PgSslMode::VerifyCa,
        "verify-full" => PgSslMode::VerifyFull,
        _ => panic!("Invalid DB_SSLMODE value"),
    };

    // สร้าง PgConnectOptions พร้อม SSL Configuration
    let mut connect_options = PgConnectOptions::new()
        .username(&db_user)
        .password(&db_pass)
        .host(&db_host)
        .port(db_port.parse::<u16>().expect("Invalid DB_PORT"))
        .database(&db_name)
        .ssl_mode(ssl_mode);

    // เพิ่มใบรับรองหากกำหนดให้ใช้งาน
    if let Ok(ssl_root_cert_path) = env::var("DB_SSL_ROOT_CERT_PATH") {
        if !ssl_root_cert_path.is_empty() {
            let ca_cert_pem = fs::read(ssl_root_cert_path).expect("Failed to read SSL root certificate");
            connect_options = connect_options.ssl_root_cert_from_pem(ca_cert_pem);
        }
    }

    if let Ok(ssl_client_cert_path) = env::var("DB_SSL_CERT_PATH") {
        if !ssl_client_cert_path.is_empty() {
            let client_cert_pem = fs::read(ssl_client_cert_path).expect("Failed to read SSL client certificate");
            connect_options = connect_options.ssl_client_cert_from_pem(&client_cert_pem);
        }
    }

    if let Ok(ssl_client_key_path) = env::var("DB_SSL_KEY_PATH") {
        if !ssl_client_key_path.is_empty() {
            let client_key_pem = fs::read(ssl_client_key_path).expect("Failed to read SSL client key");
            connect_options = connect_options.ssl_client_key_from_pem(&client_key_pem);
        }
    }

    // สร้าง Database Connection Pool
    match PgPoolOptions::new()
        .max_connections(16)
        .min_connections(8)
        .max_lifetime(Duration::from_secs(60 * 60)) // Connection อายุสูงสุด 1 ชั่วโมง
        .idle_timeout(Duration::from_secs(10 * 60)) // Connection ที่ไม่ได้ใช้จะถูกปิดอัตโนมัติหลัง 10 นาที
        .connect_with(connect_options)
        .await
    {
        Ok(pool) => {
            DB_POOL.set(pool).expect("Failed to set DB_POOL");
            info!("Database initialized successfully");
        }
        Err(err) => panic!("Failed to create connection pool: {}", err),
    }

}

pub fn get_pool() -> &'static Pool<Postgres> {
    DB_POOL.get().expect("Database pool is not initialized")
}

pub async fn query_task(script: &str) {
    let pool = get_pool();
    sqlx::query(script)
        .execute(pool)
        .await
        .expect("Failed to execute query");
}