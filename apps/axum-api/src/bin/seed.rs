use std::{env, error::Error};

use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Tidak ada DATABASE_URL di .env");

    println!("Menghubungkan ke database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("Memulai proses seeding database");
    sqlx::raw_sql(include_str!("sql/db_init_seeder.sql"))
        .execute(&pool)
        .await?;
    println!("Initial seeder berhasil...!");

    println!("Seeding selesai dengan sukses");
    Ok(())
}
