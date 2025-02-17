# Mess-Cleaner-AI

## Mess-Cleaner-AI is an AI-powered file organization tool that helps you tidy up messy folders effortlessly.

How It Works:

1. User Input – The user runs the app, specifying a folder path and an AI model name.
2. Data Analysis – The app scans the folder and generates a JSON report describing the files.
3. AI Processing – The app sends the JSON data along with a prompt to the AI, requesting an optimal file organization strategy.
4. AI Suggestion – The AI returns a structured plan with new file paths.
5. User Decision – The user reviews the AI’s suggested structure and chooses whether to apply it.

Effortless, smart, and efficient—Mess-Cleaner-AI brings order to digital chaos! 🚀

> ⚠️ **Warning:** Do not use `mess-cleaner-ai` on important files, such as passwords, confidential documents, or sensitive system files. In case of an unexpected bug or system interruption, the application may modify or remove data irreversibly. Always create backups before using it on valuable data.  
> The author is not responsible for any lost or misplaced files due to the use of this application.

## Setup

Before using this application, you need to install the following dependencies:

### Setup for macOS

1. Install or update `Xcode`.
2. Install `ollama` and launch it.
3. Download the required LLM via Ollama:

   ```sh
   ollama pull deepseek-r1:latest
   ```

   > You can find a more detailed guide here: [Ollama GitHub](https://github.com/ollama/ollama)

   It is recommended to use an LLM with a higher number of nodes for more accurate results. This project has been tested with `deepseek-r1:latest`, so if you don’t have a preference, use that model.

4. Run the application:

   ```sh
   cargo run -- -M deepseek-r1:latest --show-ai-thinking --path ./test_cases/messy-folder
   ```

## Usage

### Running the Application

To launch `mess-cleaner-ai`, use the following command:

```sh
mess-cleaner-ai --model <MODEL_NAME> --path <PATH_TO_FOLDER>
```

### Command-Line Flags

The application provides several command-line flags to configure its behavior. Below is a table listing all available flags along with their descriptions:

| Flag                 | Short | Default                               | Description                                                   |
| -------------------- | ----- | ------------------------------------- | ------------------------------------------------------------- |
| `--model`            | `-M`  | Required                              | Specifies the model name loaded in Ollama to use.             |
| `--path`             | `-P`  | Required                              | Specifies the path to the folder containing files to reorder. |
| `--recursive`        | `-R`  | `false`                               | Determines if inner folders should be processed recursively.  |
| `--show-ai-thinking` | `-A`  | `false`                               | Displays AI thinking details during execution.                |
| `--show-prompt`      | `-S`  | `false`                               | Displays the AI prompt.                                       |
| `--force-apply`      | `-F`  | `false`                               | Applies the reordering plan without requiring user review.    |
| `--server-address`   | `-n`  | `http://localhost:11434/api/generate` | Overrides the default LLM server address.                     |

### Example Usage

If you want to use it globally, while it's not published:

1. build with `cargo build --release`
2. `sudo mv target/release/mess-cleaner-ai /usr/local/bin/mess-cleaner-ai`

```sh
# Basic usage
mess-cleaner-ai --model deepseek-r1:latest --path ./documents

# Enable recursive processing and show AI thinking details
mess-cleaner-ai --model deepseek-r1:latest --path ./documents --recursive --show-ai-thinking

# Force apply changes without review
mess-cleaner-ai --model deepseek-r1:latest --path ./documents --force-apply
```

Ensure that required arguments (`--model` and `--path`) are provided for the application to function correctly.

### Model Configuration

On the first run, `mess-cleaner-ai` will create a `.mess-cleaner-ai` folder in your home directory. Inside this folder, a `config.toml` file will be generated, containing various model configuration options. By default, all configuration fields are commented out. You can uncomment and modify individual settings as needed—any fields left commented will fall back to their default values in the code.

### Prompt Configuration

The `.mess-cleaner-ai/prompts` directory contains predefined prompts that will be sent to the LLM.  
All source file paths will be appended to the end of the prompt automatically, so **do not include `{}` placeholders** in the prompt text.

You can experiment by modifying the prompts to see how they affect performance. If you discover a prompt that significantly improves results, please consider submitting a **pull request (PR)** with your suggested changes.

### Automatic Configuration Recovery

Each time you launch `mess-cleaner-ai`, it reads the latest versions of the configuration file and prompts.  
If you accidentally modify or corrupt a file, simply delete it and restart `mess-cleaner-ai`—missing configuration files will be regenerated with default values automatically.

## Contribution

Before contribution please run `bash setup-hooks.sh`.
This will create git precommit hook, which will run linters before commit.
Run `cargo clippy` to reveal code problems and `cargo fmt` to fix linting errors.
If you installed some dependencies - please run `cargo +nightly udeps` to check that all of them has been used.

## TODO

- Fix -R pathes
- Clean up plan and source.
- Handle files name collision case.
- Update rust version.
- Release.

-- tech debt --

- Rollback?
- Multiple AI requests for improvements.
- Add tests.

-- refactoring --

- separate pring messages to separated module logger
