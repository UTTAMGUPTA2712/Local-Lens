use rusqlite::{Connection, Result};
use std::path::PathBuf;

pub fn setup_db() -> Result<Connection> {
    let conn = Connection::open("image_tags.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (path TEXT PRIMARY KEY, tags TEXT)",
        [],
    )?;
    Ok(conn)
}

pub fn store_tags(conn: &Connection, path: &std::path::Path, tags: &[String]) -> Result<()> {
    let tags_str = tags.join(",");
    conn.execute(
        "INSERT OR REPLACE INTO images (path, tags) VALUES (?, ?)",
        [path.to_str().unwrap(), &tags_str],
    )?;
    Ok(())
}

pub fn search_images(conn: &Connection, query: &str) -> Result<Vec<PathBuf>> {
    let mut stmt = conn.prepare("SELECT path FROM images WHERE tags LIKE ?")?;
    let like_query = format!("%{}%", query);
    let rows = stmt.query_map([&like_query], |row| row.get::<_, String>(0))?;
    rows.map(|r| r.map(PathBuf::from)).collect()
}

pub fn get_images_with_tag(conn: &Connection, tag: &str) -> Result<Vec<(PathBuf, Vec<String>)>> {
    let mut stmt = conn.prepare("SELECT path, tags FROM images WHERE tags LIKE ?")?;
    let like_query = format!("%{}%", tag);
    let rows = stmt.query_map([&like_query], |row| {
        let path: String = row.get(0)?;
        let tags_str: String = row.get(1)?;
        let tags: Vec<String> = tags_str.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok((PathBuf::from(path), tags))
    })?;
    rows.collect()
}
