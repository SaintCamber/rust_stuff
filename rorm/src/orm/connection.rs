use::sqlx::{PgPool,SqlitePool};

pub struct DatabaseConnection{
    postgres_pool : PgPool,
    sqlite_pool : SqlitePool,
}

impl DatabaseConnection {
    
    pub async fun(pg_url: &str,sqlite_url: &str)-> Result<Self,sqlx::Error> {
        let pg_pool = PgPool::connect(pg_url).await?;
        let sqlite_pool = SqlitePool::connect(sqlite_url).await?;

        Ok(DatabaseConnection {
            pg_pool,
            sqlite_pool,
        })
    }
}