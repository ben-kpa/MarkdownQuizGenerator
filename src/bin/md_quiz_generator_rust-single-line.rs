use chrono::Local;
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn print_instructions() {
    let instructions = r#"
    This script processes a Markdown (.md) file by extracting text from specific sections.

    Instructions:
    1. When you run this script, a file dialog will open. Select the Markdown file you want to process.
    2. The script will extract text following '## ' headers until another header is encountered, ignoring other headers.
    3. The processed content will be saved to a new file with '_processed_' followed by the current timestamp appended to the original file name.
    4. The new file will be saved in the same directory as the original file.

    Note:
    - Ensure the selected file has read permissions and the destination directory has write permissions.
    "#;
    println!("{}", instructions);
}

fn get_current_datetime() -> String {
    Local::now().format("%Y%m%d%H%M%S").to_string()
}

fn process_markdown(input_file: &str) {
    let path = Path::new(input_file);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => {
            println!("Couldn't open {}: {}", display, why);
            return;
        }
        Ok(file) => file,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut extracted_text = Vec::new();
    let mut temp_text = Vec::new();
    let mut copy = false;

    for line in lines.iter() {
        if line.trim().is_empty() {
            continue; // Skip empty lines
        }
        if line.starts_with("# ") || line.starts_with("###") {
            if copy && !temp_text.is_empty() {
                extracted_text.push(temp_text.join(" "));
                temp_text.clear();
            }
            copy = false;
        } else if line.starts_with("## ") {
            if copy && !temp_text.is_empty() {
                extracted_text.push(temp_text.join(" "));
                temp_text.clear();
            }
            copy = true;
        } else if copy {
            temp_text.push(line.clone());
        }
    }
    if copy && !temp_text.is_empty() {
        extracted_text.push(temp_text.join(" "));
    }

    let output_file = path.with_file_name(format!(
        "{}_processed_{}.md",
        path.file_stem().unwrap().to_str().unwrap(),
        get_current_datetime()
    ));
    let output_file_str = output_file.to_str().unwrap();
    let mut outfile = match File::create(&output_file) {
        Err(why) => {
            println!("Couldn't create {}: {}", output_file_str, why);
            return;
        }
        Ok(outfile) => outfile,
    };

    for text in extracted_text {
        if let Err(why) = writeln!(outfile, "{}", text) {
            println!("Couldn't write to {}: {}", output_file_str, why);
            return;
        }
    }

    println!("Processed file saved as: {}", output_file_str);
}

fn main() {
    print_instructions();

    let file_path = FileDialog::new()
        .add_filter("Markdown Files", &["md"])
        .add_filter("All Files", &["*"])
        .pick_file();

    match file_path {
        Some(path) => {
            let path_str = path.to_str().unwrap();
            process_markdown(path_str);
        }
        None => {
            println!("No file selected!");
        }
    }
}
