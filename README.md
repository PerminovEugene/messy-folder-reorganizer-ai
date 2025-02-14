# Mess-Cleaner-AI

## Mess-Cleaner-AI is an AI-powered file organization tool that helps you tidy up messy folders effortlessly.

How It Works:

1. User Input – The user runs the app, specifying a folder path and an AI model name.
2. Data Analysis – The app scans the folder and generates a JSON report describing the files.
3. AI Processing – The app sends the JSON data along with a prompt to the AI, requesting an optimal file organization strategy.
4. AI Suggestion – The AI returns a structured plan with new file paths.
5. User Decision – The user reviews the AI’s suggested structure and chooses whether to apply it.

Effortless, smart, and efficient—Mess-Cleaner-AI brings order to digital chaos! 🚀

## Setup

Before using this application, you need to install the following dependencies:

### Setup for macOS

1. Install or update `Xcode`.
2. Install `cmake`.
3. Install `ollama`.
4. Download the required LLM via Ollama:

   ```sh
   ollama pull deepseek-r1:latest
   ```

   > You can find a more detailed guide here: [Ollama GitHub](https://github.com/ollama/ollama)

   It is recommended to use an LLM with a higher number of nodes for more accurate results. This project has been tested with `deepseek-r1:latest`, so if you don’t have a preference, use that model.

5. Run the application:

   ```sh
   cargo run -- -M deepseek-r1:latest --show-ai-thinking --path ./test_cases/messy-folder
   ```

## Usage

## Command-Line Flags

The application provides several command-line flags to configure its behavior. Below is a table listing all available flags along with their descriptions:

| Flag                 | Short | Default  | Description                                                   |
| -------------------- | ----- | -------- | ------------------------------------------------------------- |
| `--model`            | `-M`  | Required | Specifies the model name loaded in Ollama to use.             |
| `--path`             | `-P`  | Required | Specifies the path to the folder containing files to reorder. |
| `--recursive`        | `-R`  | `false`  | Determines if inner folders should be processed recursively.  |
| `--show-ai-thinking` | `-A`  | `false`  | Displays AI thinking details during execution.                |
| `--show-prompt`      | `-S`  | `false`  | Displays the AI prompt.                                       |
| `--force-apply`      | `-F`  | `false`  | Applies the reordering plan without requiring user review.    |

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

### Contribution

Before contribution please run `bash setup-hooks.sh`.
This will create git precommit hook, which will run linters before commit.

### TODO

1. Build and publish builded version
2. Improve CLI user experience
3. Extend flags with possiblity to configure address for ai server
4. Add tests
5. Improve CI/CD and add pre-commit hook
6. Fix warnings
7. clean up plan and souce?
8. Rollback?
9. Handle files name collision case
10. Multiple AI requests for improvements
