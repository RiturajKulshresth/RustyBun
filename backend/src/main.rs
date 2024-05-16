extern crate serde;
extern crate serde_json;
extern crate x11_clipboard;

use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use x11_clipboard::Clipboard;
// use std::path::Path;
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
struct ClipboardItem {
    id: usize,
    content: String,
}

fn main() {
    println!("Starting the clipboard rust program");

    let directory_path = "/home/ritu/Documents/RustyBun";
    if let Err(err) = fs::create_dir_all(&directory_path) {
        eprintln!("Failed to create directory: {}", err);
        return;
    }

    let clipboard = Clipboard::new().unwrap();
    let mut id_counter = load_id_counter();

    loop {
        let val = clipboard
            .load_wait(
                clipboard.setter.atoms.clipboard,
                clipboard.setter.atoms.string,
                clipboard.setter.atoms.property,
            )
            .unwrap();

        let val = String::from_utf8(val).unwrap();
        if !val.is_empty() && !val.trim().is_empty() {
            let clipboard_item = ClipboardItem {
                id: id_counter,
                content: val.clone(),
            };

            // append_to_json(&clipboard_item).expect("Failed to append to JSON");
            // println!("Copied content appended to JSON: {:?}", clipboard_item);

            let result = append_to_json(&clipboard_item);
            match result {
                Ok(_) => println!("Copied content appended to JSON: {:?}", clipboard_item),
                Err(err) => {
                    println!("Failed to append to JSON: {:?}", err);

                    let path = "/home/ritu/Documents/RustyBun/id_counter.txt";
                    let path2 = "/home/ritu/Documents/RustyBun/clipboard.json";

                    let file = File::create(path).expect("Failed to open id_counter.txt");
                    file.set_len(0).expect("Failed to truncate file");

                    let file2 = File::create(path2).expect("Failed to open clipboard.txt");
                    file2.set_len(0).expect("Failed to truncate file");
                }
            }

            id_counter += 1;
            save_id_counter(id_counter);
        }
    }
}

fn load_id_counter() -> usize {
    let path = "/home/ritu/Documents/RustyBun/id_counter.txt";
    let path2 = "/home/ritu/Documents/RustyBun/clipboard.json";
    if Path::new(path).exists() {
        let mut file = File::open(path).expect("Failed to open id_counter.txt");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read id_counter.txt");

        if contents.trim().is_empty() {
            // Return default value if the file is empty
            0
        } else {
            let count = contents
                .trim()
                .parse()
                .expect("Failed to parse id_counter.txt");

            // Check if count is >= 200
            if count >= 200 {
                // Truncate the file
                let file = File::create(path).expect("Failed to open id_counter.txt");
                file.set_len(0).expect("Failed to truncate file");

                let file2 = File::create(path2).expect("Failed to open clipboard.txt");
                file2.set_len(0).expect("Failed to truncate file");

                return 0;
            }
            count
        }
    } else {
        0
    }
}

fn save_id_counter(id_counter: usize) {
    // let path = "id_counter.txt";
    let path = "/home/ritu/Documents/RustyBun/id_counter.txt"; // Absolute path

    if !Path::new(path).exists() {
        let mut file = File::create(&path).expect("Failed to create id_counter.txt");
        write!(file, "{}", id_counter).expect("Failed to write id_counter.txt");
    } else {
        println!("File already exists: {}", path);

        // Open the file in write mode to truncate its content and rewrite
        let mut file = File::create(&path).expect("Failed to open id_counter.txt");
        file.set_len(0).expect("Failed to truncate file");

        // Write the new id_counter value
        write!(file, "{}", id_counter).expect("Failed to write id_counter.txt");
    }
}

fn append_to_json(clipboard_item: &ClipboardItem) -> io::Result<()> {
    // let path = "clipboard.json";
    let path = "/home/ritu/Documents/RustyBun/clipboard.json"; // Absolute path

    if !Path::new(path).exists() {
        let mut file = File::create(&path)?;
        file.write_all(b"[]")?;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)?;

    let mut json_data = String::new();
    file.read_to_string(&mut json_data)?;

    let mut parsed_json: Value = if json_data.trim().is_empty() {
        // If the file is empty, initialize it with an empty JSON array
        json!([])
    } else {
        // Parse existing JSON data into a serde_json::Value
        println!("cat1");
        serde_json::from_str(&json_data)?
        
    };
    println!("cat12");


    let new_entry = json!({
        "id": clipboard_item.id,
        "item": clipboard_item.content,
    });

    parsed_json.as_array_mut().unwrap().push(new_entry);

    let updated_json = serde_json::to_string(&parsed_json)?;

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;
    file.write_all(updated_json.as_bytes())?;

    Ok(())
}
