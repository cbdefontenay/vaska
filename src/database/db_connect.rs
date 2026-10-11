use std::sync::OnceLock;
use turso::{Builder, Connection, Database, Error, Result, Value};

/// URL enregistrée dans la base de données locale.
#[derive(Debug, Clone, PartialEq)]
pub struct SavedUrl {
    pub id: i64,
    pub url: String,
}

static DB: OnceLock<Database> = OnceLock::new();

#[cfg(target_os = "android")]
fn database_path() -> String {
    // Sur Android, le répertoire de travail courant n'est pas accessible en
    // écriture : on utilise le répertoire privé de l'application.

    let data_dir = "/data/data/com.cyprien.vaska/files";
    let _ = std::fs::create_dir_all(data_dir);
    format!("{data_dir}/vaska.db")
}

#[cfg(not(target_os = "android"))]
fn database_path() -> String {
    "vaska.db".to_string()
}

async fn connection() -> Result<Connection> {
    if let Some(db) = DB.get() {
        return db.connect();
    }

    let db = Builder::new_local(&database_path()).build().await?;
    let conn = db.connect()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
            id  INTEGER PRIMARY KEY AUTOINCREMENT,
            url TEXT NOT NULL
        );",
        (),
    )
    .await?;

    let _ = DB.set(db);
    Ok(conn)
}

/// Insère une nouvelle URL et retourne l'id généré.
pub async fn insert_url(url: &str) -> Result<i64> {
    let conn = connection().await?;
    conn.execute("INSERT INTO urls (url) VALUES (?1)", (url,))
        .await?;
    Ok(conn.last_insert_rowid())
}

/// Récupère toutes les URLs enregistrées, de la plus récente à la plus ancienne.
pub async fn list_urls() -> Result<Vec<SavedUrl>> {
    let conn = connection().await?;
    let mut rows = conn
        .query("SELECT id, url FROM urls ORDER BY id DESC", ())
        .await?;

    let mut saved_urls = Vec::new();
    while let Some(row) = rows.next().await? {
        let (id, url) = match (row.get_value(0)?, row.get_value(1)?) {
            (Value::Integer(id), Value::Text(url)) => (id, url),
            _ => {
                return Err(Error::ConversionFailure(
                    "types inattendus dans la table urls".to_string(),
                ));
            }
        };
        saved_urls.push(SavedUrl { id, url });
    }

    Ok(saved_urls)
}

/// Supprime une URL enregistrée.
pub async fn delete_url(id: i64) -> Result<()> {
    let conn = connection().await?;
    conn.execute("DELETE FROM urls WHERE id = ?1", (id,))
        .await?;
    Ok(())
}
