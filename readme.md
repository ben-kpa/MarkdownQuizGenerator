# Markdown Quiz Generator

## Overview

This repository contains two Rust scripts designed to process Markdown files with a specific formatting style. These scripts help users extract and manipulate content from Markdown notes that follow a structured format, making it easier to manage quiz questions and answers.

## Markdown Formatting Style

The scripts are designed to work with Markdown files that adhere to the following structure:

- **Heading 1 (#):** Represents the main topic of the document.
- **Heading 2 (##):** Represents quiz questions.
- **Heading 3 (###):** Represents the answer to the preceding quiz question. This section may contain various subsections.

### Example Markdown Structure

```markdown
# Topic 1

## Question 1
Content for question 1.

### Answer 1
Explanation or details for answer 1.

## Question 2
Content for question 2.

### Answer 2
Explanation or details for answer 2.
```

## Scripts

### `main.rs`

This script processes a Markdown file by extracting specific sections.

#### Functionality

1. **File Selection:** Opens a file dialog to select the Markdown file to process.
2. **Content Extraction:** Extracts lines starting with `#` (H1 headers) and `##` (H2 headers), as well as lines following an `##` header until another header is encountered.
3. **File Saving:** Saves the processed content to a new file with `_quiz_` followed by the current timestamp appended to the original file name. The new file is saved in the same directory as the original file.

#### Expected Output

Given the example Markdown structure, the output file will look like:

```markdown
# Topic 1

## Question 1
Content for question 1.

### try:


## Question 2
Content for question 2.

### try:
```

### `single-line-generator.rs`

This script processes a Markdown file by extracting text from specific sections.

#### Functionality

1. **File Selection:** Opens a file dialog to select the Markdown file to process.
2. **Content Extraction:** Extracts text following `##` headers until another header is encountered, ignoring other headers.
3. **File Saving:** Saves the processed content to a new file with `_processed_` followed by the current timestamp appended to the original file name. The new file is saved in the same directory as the original file.

#### Expected Output

Given the example Markdown structure, the output file will look like:

```markdown
Content for question 1.
Content for question 2.
```

## Instructions

### Running the Scripts

1. **Execute the Script:** Run the respective script executable.
2. **Select Markdown File:** A file dialog will open. Select the Markdown file you want to process.
3. **Processing:** The script will process the file based on the described functionality.
4. **Output:** The processed content will be saved in a new file in the same directory as the original file.

### Note

- Ensure the selected file has read permissions and the destination directory has write permissions.

## Releases

The latest releases include executables for both scripts:

- **main_exe**
- **single-line-generator_exe**

These executables can be run directly on your system without needing to compile the Rust code. The releases are available for Windows only. Download the latest releases from the repository's [Releases](https://github.com/your-repo/releases) page.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project uses the `rfd` crate for file dialogs and `chrono` crate for date and time management. Special thanks to the contributors of these projects.

---

For any issues or feature requests, please open an issue in the repository. Contributions are welcome!
