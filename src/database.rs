use rusqlite::{Connection, params, Result};
use uuid::Uuid;
use crate::Note;

pub fn create_database() -> Result<()> {
    let conn = Connection::open("./database.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Notes (
            id TEXT PRIMARY KEY,
            note TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn add_note(note: Note) -> Result<String> {
    let conn = Connection::open("./database.db")?;
    conn.execute(
        "INSERT INTO Notes (id, note) VALUES(?, ?)",
        params![note.uuid.to_string(), note.text],
    )?;
    Ok("Note added successfully".to_string())
}

pub fn get_notes() -> Result<String> {
    let conn = Connection::open("./database.db")?;

    let mut stmt = conn.prepare("SELECT id, note FROM Notes")?;
    let notes_iter = stmt.query_map([], |row| {
        Ok(Note {
            uuid: Uuid::parse_str(&row.get::<_, String>(0)?).unwrap(),
            text: row.get(1)?,
        })
    })?;

    let mut notes_str = String::new();
    for note in notes_iter {
        let note = note?; // Handle potential error from query_map
        notes_str += &format!("{}: {}\n", note.uuid, note.text);
    }

    Ok(notes_str)
}
