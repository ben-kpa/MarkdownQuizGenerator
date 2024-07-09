use chrono::Local;
use rfd::FileDialog;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn print_instructions() {
    let instructions = r#"
    This script processes a Markdown (.md) file by extracting specific sections.

    Instructions:
    1. When you run this script, a file dialog will open. Select the Markdown file you want to process.
    2. The script will extract lines starting with '# ' (H1 headers) and '## ' (H2 headers), 
       as well as lines following an '## ' header until another header is encountered.
    3. The processed content will be saved to a new file with '_quiz_' followed by the current timestamp appended to the original file name.
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
    let mut selected_lines = Vec::new();
    let mut previous_was_h2 = false;
    let mut copy = false;

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("# ") {
            if previous_was_h2 {
                selected_lines.push("### try:\n\n\n".to_string());
            }
            selected_lines.push(line.clone());
            previous_was_h2 = false;
            copy = false;
        } else if line.starts_with("## ") {
            if previous_was_h2 {
                selected_lines.push("### try:\n\n\n".to_string());
            }
            selected_lines.push(line.clone());
            previous_was_h2 = true;
            copy = true;
        } else if line.starts_with("###") {
            copy = false;
        } else if copy {
            selected_lines.push(line.clone());
        }

        if copy && (i + 1 < lines.len()) {
            let next_line = &lines[i + 1];
            if next_line.starts_with("## ") || next_line.starts_with("# ") {
                copy = false;
            }
        }
    }

    if previous_was_h2 {
        selected_lines.push("### try:\n\n\n".to_string());
    }

    let output_file = path.with_file_name(format!(
        "{}_quiz_{}.md",
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

    for line in selected_lines {
        if let Err(why) = writeln!(outfile, "{}", line) {
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
