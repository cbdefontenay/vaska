use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::{query, query_as, Error, SqlitePool};
use std::sync::OnceLock;
use std::{fs::create_dir_all, str::FromStr};

/// URL enregistrée dans la base de données locale.
#[derive(Debug, Clone, PartialEq)]
pub struct SavedUrl {
    pub id: i64,
    pub url: String,
}

static POOL: OnceLock<SqlitePool> = OnceLock::new();

#[cfg(target_os = "android")]
fn database_url() -> String {
    // Sur Android, le répertoire de travail courant n'est pas accessible en
    // écriture : on utilise le répertoire privé de l'application.

    let data_dir = "/data/data/com.cyprien.vaska/files";
    let _ = create_dir_all(data_dir);
    format!("sqlite://{data_dir}/vaska.db?mode=rwc")
}

#[cfg(not(target_os = "android"))]
fn database_url() -> String {
    "sqlite://vaska.db?mode=rwc".to_string()
}

async fn pool() -> Result<SqlitePool, Error> {
    if let Some(pool) = POOL.get() {
        return Ok(pool.clone());
    }

    let options =
        SqliteConnectOptions::from_str(&database_url()).expect("URL de base de données invalide");
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(options)
        .await?;

    query(
        "CREATE TABLE IF NOT EXISTS urls (
            id  INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    let _ = POOL.set(pool.clone());
    Ok(pool)
}

/// Insère une nouvelle URL et retourne l'id généré.
pub async fn insert_url(url: &str) -> Result<i64, Error> {
    let pool = pool().await?;
    let result = query("INSERT INTO urls (url) VALUES (?1)")
        .bind(url)
        .execute(&pool)
        .await?;
    Ok(result.last_insert_rowid())
}

/// Récupère toutes les URLs enregistrées, de la plus récente à la plus ancienne.
pub async fn list_urls() -> Result<Vec<SavedUrl>, Error> {
    let pool = pool().await?;
    let rows = query_as::<_, (i64, String)>("SELECT id, url FROM urls ORDER BY id DESC")
        .fetch_all(&pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|(id, url)| SavedUrl { id, url })
        .collect())
}

/// Supprime une URL enregistrée.
pub async fn delete_url(id: i64) -> Result<(), Error> {
    let pool = pool().await?;
    query("DELETE FROM urls WHERE id = ?1")
        .bind(id)
        .execute(&pool)
        .await?;
    Ok(())
}
