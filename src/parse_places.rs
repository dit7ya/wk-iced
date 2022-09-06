extern crate rusqlite;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Entry {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
}

pub fn get_firefox_entries() -> Result<Vec<Entry>> {
    let path = "./places.sqlite"; // NOTE: wrt the root dir

    let conn = Connection::open(path)?;

    let mut stmt = conn.prepare("SELECT id, url, title FROM moz_places")?;

    let rows = stmt.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            url: row.get(1)?,
            title: row.get(2)?,
        })
    })?;
    let mut entries = Vec::new();

    let limit = 1000;
    let mut count = 0;
    for row in rows {
        if count < limit {
            entries.push(row?);
            count += 1;
        } else {
            break;
        }
    }

    Ok(entries)
}
