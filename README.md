# messy-folder-reorganizer-ai

## messy-folder-reorganizer-ai is an AI-powered file organization tool that helps you tidy up messy folders effortlessly.

How It Works:

1. User Input – The user runs the app, specifying a folder path and an AI model name.
2. Data Analysis – The app scans the folder and generates a JSON report describing the files.
3. AI Processing – The app sends the JSON data along with a prompt to the AI, requesting an optimal file organization strategy.
4. AI Suggestion – The AI returns a structured plan with new file paths.
5. User Decision – The user reviews the AI’s suggested structure and chooses whether to apply it.

Effortless, smart, and efficient—messy-folder-reorganizer-ai brings order to digital chaos! 🚀

> ⚠️ **Warning:** Do not use `messy-folder-reorganizer-ai` on important files, such as passwords, confidential documents, or sensitive system files. In case of an unexpected bug or system interruption, the application may modify or remove data irreversibly. Always create backups before using it on valuable data.  
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

4. Download latest release

   ```sh
   curl -s https://api.github.com/repos/PerminovEugene/messy-folder-reorganizer-ai/releases/latest | \
     grep "browser_download_url.*messy-folder-reorganizer-ai-aarch64-apple-darwin.tar.gz" | \
     cut -d '"' -f 4 | \
     xargs curl -L -o messy-folder-reorganizer-ai-macos.tar.gz
   ```

5. Extract the file

   ```sh
   tar -xvzf messy-folder-reorganizer-ai-macos.tar.gz
   ```

6. Move to `/usr/local/bin` for system-wide use

   ```sh
   sudo mv messy-folder-reorganizer-ai /usr/local/bin/messy-folder-reorganizer-ai
   ```

7. Verify installation

   ```sh
   messy-folder-reorganizer-ai --help
   ```

### Build from sources

If you want to build it from sources by yourself:

1. Pull repository

   ```sh
   git clone git@github.com:PerminovEugene/messy-folder-reorganizer-ai.git
   ```

2. Build project with

   ```sh
   cargo build --release
   ```

3. Launch with

   ```sh
   cargo run -- -M deepseek-r1:latest --show-ai-thinking  --path ./../../Documents/ -S
   ```

## Usage

### Running the Application

To launch `messy-folder-reorganizer-ai`, use the following command:

```sh
messy-folder-reorganizer-ai --model <MODEL_NAME> --path <PATH_TO_FOLDER>
```

### Command-Line Flags

The application provides several command-line flags to configure its behavior. Below is a table listing all available flags along with their descriptions:

| Flag                     | Short | Default                               | Description                                                   |
| ------------------------ | ----- | ------------------------------------- | ------------------------------------------------------------- |
| `--model`                | `-M`  | Required                              | Specifies the model name loaded in Ollama to use.             |
| `--path`                 | `-P`  | Required                              | Specifies the path to the folder containing files to reorder. |
| `--recursive`            | `-R`  | `false`                               | Determines if inner folders should be processed recursively.  |
| `--show-ai-thinking`     | `-A`  | `false`                               | Displays AI thinking details during execution.                |
| `--show-prompt`          | `-S`  | `false`                               | Displays the AI prompt.                                       |
| `--force-apply`          | `-F`  | `false`                               | Applies the reordering plan without requiring user review.    |
| `--server-address`       | `-n`  | `http://localhost:11434/api/generate` | Overrides the default LLM server address.                     |
| `--skip-problematic-dir` | `-d`  | `false`                               | Will skip problematic directories and files.                  |

### Usage examples

```sh
# Basic usage
messy-folder-reorganizer-ai -M deepseek-r1:latest -P ./../../Downloads -S -A

# Enable recursive processing and show AI thinking details
messy-folder-reorganizer-ai --model deepseek-r1:latest --path ./documents --recursive --show-ai-thinking

# Force apply changes without review
messy-folder-reorganizer-ai --model deepseek-r1:latest --path ./documents --force-apply
```

Ensure that required arguments (`--model` and `--path`) are provided for the application to function correctly.

### Model Configuration

On the first run, `messy-folder-reorganizer-ai` will create a `.messy-folder-reorganizer-ai` folder in your home directory. Inside this folder, a `config.toml` file will be generated, containing various model configuration options. By default, all configuration fields are commented out and won't be sent. You can uncomment and modify individual settings as needed—any fields left commented will fall back to their default values in the code.

More information about the parameters can be found [https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values](here).

It is recommended to specify `num_ctx` in `config.toml, as the result heavily depends on the context size.

### Prompt Configuration

The `.messy-folder-reorganizer-ai/prompts` directory contains predefined prompts that will be sent to the LLM.  
All source file paths will be appended to the end of the prompt automatically, so **do not include `{}` placeholders** in the prompt text.

You can experiment by modifying the prompts to see how they affect performance. If you discover a prompt that significantly improves results, please consider submitting a **pull request (PR)** with your suggested changes.

### Automatic Configuration Recovery

Each time you launch `messy-folder-reorganizer-ai`, it reads the latest versions of the configuration file and prompts.  
If you accidentally modify or corrupt a file, simply delete it and restart `messy-folder-reorganizer-ai`—missing configuration files will be regenerated with default values automatically.

## Contribution

Before contribution please run `bash setup-hooks.sh`.
This will create git precommit hook, which will run linters before commit.
Run `cargo clippy` to reveal code problems and `cargo fmt` to fix linting errors.
If you installed some dependencies - please run `cargo +nightly udeps` to check that all of them has been used.

## Uninstall & Purge

To completely remove `messy-folder-reorganizer-ai` from your system:

```sh
rm -f /usr/local/bin/messy-folder-reorganizer-ai
rm -rf ~/.messy-folder-reorganizer-ai
```

## TODO

### Next releases debt and ideas

#### V2 implementation plan

- Quadrant distribution
- Add quadrant compatible license
- Save string (files) as vectors
- get vectors

- Multiple AI requests for result improvements.
- Improve processing huge folders (batches in parallel + initial request with files formats).
- Add optional destination folder.
- Filtration by formats?
- Handle file names collision case.
- Rollback by plan.json
- Improve error handling.
- Add tests.
- Update rust version.
- Enable cross platform builds if somebody will be interested

### Refactoring ideas

- Move print messages to separated logger module

## v2

- change model to embedding one (or load additionallly), from here https://ollama.com/blog/embedding-models
- embeddings size from ollama model should be the same as size of qdrant collection, so it should be configurable
- Allow measure configuration cosin || euclide || dot?
- Allow max distance configuration

### workflow

1. parse dest, get embeddings, save embeddings into qdrant
2. parse source, find closest vector in db
3. Add dest path (if not home) to vectors, may be put into dest root?
4. if distance between closest vectors is not enough, put to unknown
5. move valid files to valid suggestions

6. generate folder names
