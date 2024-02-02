use std::{env, fs::File, io::{Read, Write, Seek, SeekFrom}, io};
use reqwest::Client;
use toml_edit::{Document, value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Determine the directory of the current executable
    let mut config_path = env::current_exe()?;
    config_path.pop(); // Remove the executable name, leaving just the directory
    config_path.push("Config.toml"); // Append 'Config.toml' to the directory path

    let mut file = File::open(&config_path).or_else(|_| -> io::Result<File> {
        let mut file = File::create(&config_path)?;
        file.write_all(b"[api]\nurl = \"\"")?;
        file.sync_all()?;
        Ok(file)
    })?;

    file.seek(SeekFrom::Start(0))?; // Ensure we read from the start
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut doc = contents.parse::<Document>()?;

    match args.get(1).map(String::as_str) {
        Some("new") if args.len() > 2 => {
            let text = args[2].clone();
            println!("NEW NOTE: {}", text);
            send_new_note(&doc, text).await?;
        },
        Some("get") => {
            get_notes(&doc).await?;
        },
        Some("seturl") if args.len() > 2 => {
            let url = args[2].as_str();
            if let Some(api) = doc["api"].as_table_mut() {
                api.insert("url", value(url));
                // Write changes back to the file
                file.seek(SeekFrom::Start(0))?; // Seek to start to overwrite
                file.set_len(0)?; // Clear the file before writing
                file.write_all(doc.to_string().as_bytes())?;
            }
        }
        _ => {
            println!("I DON'T KNOW THIS COMMAND!");
        }
    }

    Ok(())
}

async fn send_new_note(doc: &Document, note: String) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = doc["api"]["url"].as_str().unwrap_or("http://localhost:2222/new_note?text=");
    let res = client.post(format!("{}{}", url, note.replace(" ", "+")))
        .send()
        .await?
        .text()
        .await?;

    println!("{}", res);
    Ok(())
}

async fn get_notes(doc: &Document) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = doc["api"]["url"].as_str().unwrap_or("http://localhost:2222/get_notes");
    let resp = client.get(url)
        .send()
        .await?
        .text()
        .await?;

    println!("{}", resp);
    Ok(())
}
