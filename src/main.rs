use sqlx::MySqlPool;
use sqlx::mysql::{MySqlConnectOptions, MySqlSslMode};
use std::path::PathBuf;

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    id: i64,
    name: String,
    email: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn get_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, email, created_at, updated_at FROM users"
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

async fn insert_test_users(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let test_users = vec![
        ("太郎", "taro@example.com"),
        ("花子", "hanako@example.com"),
        ("次郎", "jiro@example.com"),
        ("美咲", "misaki@example.com"),
        ("健太", "kenta@example.com"),
    ];

    for (name, email) in test_users {
        sqlx::query!(
            "INSERT INTO users (name, email, created_at, updated_at) VALUES (?, ?, NOW(), NOW())",
            name,
            email
        )
        .execute(pool)
        .await?;

        println!("Inserted user: {} ({})", name, email);
    }

    Ok(())
}

fn get_cert_path() -> Option<PathBuf> {
    std::env::var("CA_CERT_NAME")
        .map(|cert_name| {
            std::env::var("CERT_DIR")
                .map(|cert_dir| PathBuf::from(cert_dir).join(cert_name))
                .expect("Failed to get certificate directory")
        })
        .ok()
}

async fn create_database_pool() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    let ca_cert_path = get_cert_path();

    println!("Using certificate at: {:?}", ca_cert_path);

    let options = MySqlConnectOptions::new()
        .host(&std::env::var("DATABASE_HOST")?)
        .port(
            std::env::var("DATABASE_PORT")
                .map(|port| port.parse().expect("Failed to parse DATABASE_PORT"))
                .expect("DATABASE_PORT not set"),
        )
        .username(&std::env::var("DATABASE_USERNAME")?)
        .password(&std::env::var("DATABASE_PASSWORD")?)
        .database(&std::env::var("DATABASE_NAME")?);

    let options = if let Some(ca_cert_path) = ca_cert_path {
        options.ssl_mode(MySqlSslMode::VerifyIdentity).ssl_ca(&ca_cert_path)
    } else {
        options
    };

    let pool = sqlx::MySqlPool::connect_with(options).await?;
    Ok(pool)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = create_database_pool().await?;

    println!("Inserting test users...");
    match insert_test_users(&pool).await {
        Ok(()) => println!("Successfully inserted test users"),
        Err(e) => eprintln!("Error inserting test users: {}", e),
    }

    println!("\nFetching all users...");
    match get_all_users(&pool).await {
        Ok(users) => {
            println!("Found {} users:", users.len());
            for user in users {
                println!("{:?}", user);
            }
        },
        Err(e) => {
            eprintln!("Error fetching users: {}", e);
        },
    }

    pool.close().await;
    Ok(())
}
