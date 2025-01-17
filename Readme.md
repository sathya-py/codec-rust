**Code Collator**

This is a Rust application that helps you find and summarize code files within a specified directory.

**Features:**

* Finds files based on user-defined extensions (e.g., `.txt`, `.py`, `.java`).
* Allows skipping specific extensions and folders.
* Generates a summary report with file paths and content (optional).
* Supports parallel processing for faster execution (optional).

**Installation**

**Prerequisites:**

* Rust compiler (version 1.56 or later): [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

**Building and Running:**

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/code-collator.git
   ```

2. Navigate to the project directory:

   ```bash
   cd code-collator
   ```

3. Build the application:

   ```bash
   cargo build --release
   ```

   This will create an executable named `target/release/code_collator` (or similar depending on your operating system).

4. Run the application:

   ```bash
   ./target/release/code_collator <directory> [options]
   ```

   Replace `<directory>` with the path to the directory you want to search.

**Options:**

* `-o`, `--output`: Specify the output file name (default: summary.txt).
* `-s`, `--skip`: List of file extensions to skip (e.g., -s ".jpg .png").
* `-e`, `--extensions`: List of valid extensions to include (e.g., -e ".py .java .cs").
* `--full-path`: Include full file paths in the output.
* `--skip-folders`: List of folder names to skip (e.g., --skip-folders "node_modules .git").

**Example Usage:**

```bash
./target/release/code_collator ./my_code -o code_summary.txt -s ".md .html" --full-path
```

This command will search for files with extensions `.py`, `.java`, and `.cs` (excluding `.md` and `.html`) within the `./my_code` directory, create a summary report named `code_summary.txt`, and include full file paths in the output.

**Contributing**

We welcome contributions to this project! Please feel free to fork the repository, make changes, and submit pull requests.

**License**

This project is licensed under the MIT License. See the `LICENSE` file for details.
