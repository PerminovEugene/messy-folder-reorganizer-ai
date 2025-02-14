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

0. Install or update `Xcode`.
1. Install `cmake`.
2. Install `ollama`.
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

```sh
# Basic usage
my_app --model gpt-4 --path ./documents

# Enable recursive processing and show AI thinking details
my_app --model gpt-4 --path ./documents --recursive --show-ai-thinking

# Force apply changes without review
my_app --model gpt-4 --path ./documents --force-apply
```

Ensure that required arguments (`--model` and `--path`) are provided for the application to function correctly.
