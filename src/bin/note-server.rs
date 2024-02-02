extern crate note;

use axum::{
    extract::Query, response::{IntoResponse, Response}, routing::{delete, get, post, put}, Router, http::StatusCode,
};
use uuid::Uuid;

use note::{database, Note, NewNote};

#[tokio::main]
async fn main() {
    if let Err(e) = database::create_database() {
        eprintln!("Failed to create database: {}", e);
        return;
    }

    let app = Router::new()
        .route("/", get(|| async { "Works" }))
        .route("/new_note", post(new_note))
        .route("/get_notes", get(get_notes))
        .route("/edit_note", put(edit_notes))
        .route("/delete_note", delete(delete_note));

        let listener = tokio::net::TcpListener::bind("0.0.0.0:2222").await.unwrap();
        axum::serve(listener, app).await.unwrap();
}

async fn new_note(Query(new_note): Query<NewNote>) -> impl IntoResponse {
    println!("POST /new_note\ntext: {}", new_note.text);

    let note = Note {
        uuid: Uuid::new_v4(),
        text: new_note.text,
    };

    match database::add_note(note) {
        Ok(_) => Response::builder().status(StatusCode::CREATED).body::<String>("Note added successfully".into()).unwrap(),
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Failed to add note: {}", e).into()).unwrap(),
    }
}

async fn get_notes() -> impl IntoResponse {
    match database::get_notes() {
        Ok(notes) => notes.into_response(),
        Err(e) => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(format!("Failed to retrieve notes: {}", e).into()).unwrap(),
    }
}

async fn edit_notes() -> String {
    "edit_note".to_string()
}

async fn delete_note() -> String {
    "delete_note".to_string()
}
