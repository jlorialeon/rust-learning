use sqlx::{postgres::PgPool, Error};
use dotenv::dotenv; // Optional

#[derive(Debug, FromRow)]
struct User {
    id: i32,
    name: String,
    last_name: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    // Read the connection string from the environment variable
    let database_url = "postgresql://postgres:ccA53150@localhost:5432/postgres".to_string();
    
    // Create a connection pool
    let pool = PgPool::connect(&database_url).await?;

    println!("Successfully connected to the database!");

    // Execute the SELECT query and map the results to the User struct
    let users = sqlx::query_as!(
        User,
        "SELECT id, name, last_name FROM \"user\""
    )
    .fetch_all(&pool)
    .await?;

    // Iterate over the results and print each user
    for user in users {
        println!("User: {:?}", user);
    }

    Ok(())
}