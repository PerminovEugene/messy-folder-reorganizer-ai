# messy-folder-reorganizer-ai

## AI-powered tool to help you clean up messy folders effortlessly

### Overview

**messy-folder-reorganizer-ai** is an AI-powered file organization tool that helps you sort and tidy up disorganized folders with minimal effort.

### How It Works

1. **User Input** – The user runs the app and provides:

   - a **source folder** path containing the files to organize
   - a **destination folder** path where organized files will be placed
   - an **AI model name** (loaded in Ollama) used to generate folder names
   - an **embedding model name** (also loaded in Ollama) used to generate vector embeddings

2. **Destination Folder Scan**

   - The app scans the destination folder and generates embeddings for each folder name.
   - These embeddings are stored in a **Qdrant** vector database.

3. **Source Folder Scan**

   - The app scans the source folder and generates embeddings for each file name.
   - It compares each file’s embedding to existing folder embeddings in the database.
   - Files without a sufficiently close match are marked for further processing.

4. **Clustering & AI Folder Naming**

   - Unmatched file embeddings are grouped using **agglomerative hierarchical clustering**.
   - Each cluster is sent to the LLM to generate a suggested folder name.

5. **Preview Results**

   - A table is displayed showing the proposed destination for each file.

6. **User Decision**
   - The user reviews the suggested structure and decides whether to apply the changes.

> ⚠️ **Warning:** Do not use `messy-folder-reorganizer-ai` on important files such as passwords, confidential documents, or critical system files.  
> In the event of a bug or interruption, the app may irreversibly modify or delete files. Always create backups before using it on valuable data.  
> The author assumes no responsibility for data loss or misplaced files caused by this application.

## Setup

### macOS Installation

1. Install or update **Xcode**.
2. Install **Ollama** and start the service.
3. Download the required LLM via Ollama:

   ```sh
   ollama pull deepseek-r1:latest
   ```

   > Recommended: Use models with a higher number of parameters for better accuracy.  
   > This project has been tested with `deepseek-r1:latest` (4.7 GB, 7.6B params).

4. Download the embedding model:

   ```sh
   ollama pull mxbai-embed-large:latest
   ```

5. Launch Qdrant vector database (easiest via Docker):

   ```sh
   docker pull qdrant/qdrant
   docker run -p 6333:6333 \
     -v $(pwd)/path/to/data:/qdrant/storage \
     qdrant/qdrant
   ```

6. Download the latest app release:

   ```sh
   curl -s https://api.github.com/repos/PerminovEugene/messy-folder-reorganizer-ai/releases/latest | \
     grep "browser_download_url.*messy-folder-reorganizer-ai-aarch64-apple-darwin.tar.gz" | \
     cut -d '"' -f 4 | \
     xargs curl -L -o messy-folder-reorganizer-ai-macos.tar.gz
   ```

7. Extract and install:

   ```sh
   tar -xvzf messy-folder-reorganizer-ai-macos.tar.gz
   sudo mv messy-folder-reorganizer-ai /usr/local/bin/messy-folder-reorganizer-ai
   ```

8. Verify the installation:

   ```sh
   messy-folder-reorganizer-ai --help
   ```

> Additional help:
>
> - [Ollama GitHub](https://github.com/ollama/ollama)
> - [Embedding Models with Ollama](https://ollama.com/blog/embedding-models)
> - [Qdrant Docs](https://qdrant.tech/documentation/guides/installation/)

## Build from Source

1. Clone the repository:

   ```sh
   git clone git@github.com:PerminovEugene/messy-folder-reorganizer-ai.git
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Run it:

   ```sh
   cargo run -- \
     -E mxbai-embed-large \
     -L deepseek-r1:latest \
     -S ./test_cases/clustering/messy-folder \
     -D ./test_cases/clustering/structured-folder
   ```

> The `./test_cases/` folder contains sample files to explore the tool’s functionality.

## Usage

### Run the App

```sh
messy-folder-reorganizer-ai \
  -E <EMBEDDING_MODEL_NAME> \
  -L <LLM_MODEL_NAME> \
  -S <SOURCE_FOLDER_PATH> \
  -D <DESTINATION_FOLDER_PATH>
```

### Command-Line Arguments

| Flag                     | Short | Default                   | Description                                                                                |
| ------------------------ | ----- | ------------------------- | ------------------------------------------------------------------------------------------ |
| `--language-model`       | `-L`  | _required_                | Language model name loaded in Ollama for folder name generation.                           |
| `--embedding-model`      | `-E`  | _required_                | Embedding model name used for generating file and folder embeddings.                       |
| `--source`               | `-S`  | _required_                | Path to the folder with files to organize.                                                 |
| `--destination`          | `-D`  | `home`                    | Path for the organized output. Defaults to the user's home directory.                      |
| `--recursive`            | `-R`  | `false`                   | Whether to scan the source folder recursively (destination is always scanned recursively). |
| `--force-apply`          | `-F`  | `false`                   | Automatically apply the reorganization plan without user confirmation.                     |
| `--skip-problematic-dir` | `-d`  | `false`                   | Skip problematic directories or files instead of stopping execution.                       |
| `--llm-address`          | `-n`  | `http://localhost:11434/` | Override the default LLM server address.                                                   |
| `--qdrant-address`       | `-q`  | `http://localhost:6334/`  | Override the default Qdrant server address.                                                |

## Configuration

### Model & ML Configuration

On the first run, the app creates a `.messy-folder-reorganizer-ai/` directory in your home folder containing:

- llm_config.toml – LLM model request configuration options
- embeddings_config.toml – Embedding model request configuration options
- rag_ml_config.toml – RAG and ML behavior settings

Model request configurations are commented out by default and will fall back to built-in values unless edited.

More information about LLM and Embedding model configuration options can be found [https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values](here).

RAG and ML configuration parameters are required and should always be present in rag_ml_config.toml.
You also can set up ignore lists for destionation and source pathes in that config file.

### Prompt Customization

Prompts are stored in:

```sh
~/.messy-folder-reorganizer-ai/prompts/
```

You can edit these to experiment with different phrasing.  
The source file list will be appended automatically, so **do not** use `{}` or other placeholders in the prompt.

Feel free to contribute improved prompts via PR!

### Auto-Recovery

If you break or delete any config/prompt files, simply re-run the app — missing files will be regenerated with default values.

## Contributing

1. Run the setup script before contributing:

   ```sh
   bash setup-hooks.sh
   ```

2. Lint & format code:

   ```sh
   cargo clippy
   cargo fmt
   ```

3. Check for unused dependencies:

   ```sh
   cargo +nightly udeps
   ```

## Uninstall & Purge

```sh
rm -f /usr/local/bin/messy-folder-reorganizer-ai
rm -rf ~/.messy-folder-reorganizer-ai
```

## TODO

### V2 Backlog

- Handle filename collisions
- Add rollback via `plan.json`
- Improve error handling
- Add tests
- Update Rust version
- Add cross-platform builds (on demand)

### Upcoming

- Add ignore list configuration
- Publish a blog article
- Add workflow diagram/image
