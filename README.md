<!-- [![codecov](https://codecov.io/gh/PerminovEugene/messy-folder-reorganizer-ai/branch/main/graph/badge.svg)](https://codecov.io/gh/PerminovEugene/messy-folder-reorganizer-ai) -->

![Build](https://img.shields.io/github/actions/workflow/status/PerminovEugene/messy-folder-reorganizer-ai/ci.yml?branch=main)
![License](https://img.shields.io/github/license/PerminovEugene/messy-folder-reorganizer-ai)
![Language](https://img.shields.io/github/languages/top/PerminovEugene/messy-folder-reorganizer-ai)
![Local AI](https://img.shields.io/badge/AI-local--only-green?logo=ai)

## messy-folder-reorganizer-ai - 🤖 AI-powered CLI for file reorganization.

### Runs fully locally — no data leaves your machine.

### How It Works

CLI supports multiple commands:

#### Process

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

#### Apply

If you decided to not apply changes after `process`, you can apply changes later with `apply` command. It expects that you didn't change files locations. This command applied migrations from the latest succesfull `process` launch.

#### Rollback

For the case if after files migrations you are changed your mind and want to return everything back.

> ⚠️ **Warning:** Do not use `messy-folder-reorganizer-ai` on important files such as passwords, confidential documents, or critical system files.  
> In the event of a bug or interruption, the app may irreversibly modify or delete files. Always create backups before using it on valuable data.  
> The author assumes no responsibility for data loss or misplaced files caused by this application.

## Small articles for the curious minds

📌 [Adding RAG & ML to the CLI](https://dev.to/evgeniiperminov/adding-rag-and-ml-to-ai-files-reorganization-cli-messy-folder-reorganizer-ai-1d3)

📌 [How cosine similarity helped files find their place](https://dev.to/evgeniiperminov/how-cosine-similarity-helped-my-cli-decide-where-files-belong-messy-folder-reorganizer-ai-fm3)

📌 [Teaching embeddings to understand folders](https://dev.to/evgeniiperminov/making-embeddings-understand-files-and-folders-with-simple-sentences-messy-folder-reorganizer-ai-mjg)

📌 [Hierarchical clustering for file grouping](https://dev.to/evgeniiperminov/embeddings-clustering-with-agglomerative-hierarchical-clustering-messy-folder-reorganizer-ai-520k)

## Setup

1. Install core developer tools

- macOS

  ```
  Install or update **Xcode**
  ```

- Linux x86_64

  ```sh
  sudo apt update
  sudo apt install -y build-essential
  ```

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

- Apple Silicon (macOS ARM64):

  ```sh
  curl -s https://api.github.com/repos/PerminovEugene/messy-folder-reorganizer-ai/releases/tags/v0.2.0 | \
    grep "browser_download_url.*messy-folder-reorganizer-ai-v0.2.0-aarch64-apple-darwin.tar.gz" | \
    cut -d '"' -f 4 | \
    xargs curl -L -o messy-folder-reorganizer-ai-macos-arm64.tar.gz
  ```

- Intel Mac (macOS x86_64):

  ```sh
  curl -s https://api.github.com/repos/PerminovEugene/messy-folder-reorganizer-ai/releases/tags/v0.2.0 | \
    grep "browser_download_url.*messy-folder-reorganizer-ai-v0.2.0-x86_64-apple-darwin.tar.gz" | \
    cut -d '"' -f 4 | \
    xargs curl -L -o messy-folder-reorganizer-ai-macos-x64.tar.gz
  ```

- Linux x86_64:

  ```sh
  curl -s https://api.github.com/repos/PerminovEugene/messy-folder-reorganizer-ai/releases/tags/v0.2.0 | \
    grep "browser_download_url.*messy-folder-reorganizer-ai-v0.2.0-x86_64-unknown-linux-gnu.tar.gz" | \
    cut -d '"' -f 4 | \
    xargs curl -L -o messy-folder-reorganizer-ai-linux-x64.tar.gz
  ```

7. Extract and install:

- Apple Silicon (macOS ARM64):

  ```sh
  tar -xvzf messy-folder-reorganizer-ai-macos-arm64.tar.gz
  sudo mv messy-folder-reorganizer-ai /usr/local/bin/messy-folder-reorganizer-ai
  ```

- Intel Mac (macOS x86_64):

  ```sh
  tar -xvzf messy-folder-reorganizer-ai-macos-x64.tar.gz
  sudo mv messy-folder-reorganizer-ai /usr/local/bin/messy-folder-reorganizer-ai
  ```

- Linux x86_64:

  ```sh
  tar -xvzf messy-folder-reorganizer-ai-linux-x64.tar.gz
  sudo mv messy-folder-reorganizer-ai /usr/local/bin/messy-folder-reorganizer-ai
  ```

8. Verify the installation:

   ```sh
   messy-folder-reorganizer-ai --help
   ```

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

## Usage

### Run the App

```sh
messy-folder-reorganizer-ai process \
  -E <EMBEDDING_MODEL_NAME> \
  -L <LLM_MODEL_NAME> \
  -S <SOURCE_FOLDER_PATH> \
  -D <DESTINATION_FOLDER_PATH>
```

```sh
messy-folder-reorganizer-ai apply \
  -i <SESSION_ID>
```

```sh
messy-folder-reorganizer-ai rollback \
 -i <SESSION_ID>
```

## Command-Line Arguments

The CLI supports the following subcommands:

---

### `process`

Processes source files, finds best-matching destination folders using embeddings, and generates a migration plan.

| Argument                  | Short | Default                  | Description                                                                          |
| ------------------------- | ----- | ------------------------ | ------------------------------------------------------------------------------------ |
| `--language-model`        | `-L`  | _required_               | Ollama LLM model name used to generate semantic folder names.                        |
| `--embedding-model`       | `-E`  | _required_               | Embedding model used for representing folder and file names as vectors.              |
| `--source`                | `-S`  | _required_               | Path to the folder with unorganized files.                                           |
| `--destination`           | `-D`  | `home`                   | Path to the folder where organized files should go.                                  |
| `--recursive`             | `-R`  | `false`                  | Whether to scan subfolders of the source folder recursively.                         |
| `--force-apply`           | `-F`  | `false`                  | Automatically apply changes after processing without showing preview.                |
| `--continue-on-fs-errors` | `-C`  | `false`                  | Allow skipping files/folders that throw filesystem errors (e.g., permission denied). |
| `--llm-address`           | `-n`  | `http://localhost:11434` | Address of the local or remote Ollama LLM server.                                    |
| `--qdrant-address`        | `-q`  | `http://localhost:6334`  | Address of the Qdrant vector database instance.                                      |

---

### `apply`

Applies a previously saved migration plan using the session ID.
Session Id will be printed during `process` execution.

| Argument       | Short | Description                                        |
| -------------- | ----- | -------------------------------------------------- |
| `--session-id` | `-i`  | The session ID generated by the `process` command. |

---

### 🔙 `rollback`

Rolls back a previously applied migration using the session ID.
Session Id will be printed during `process` execution.

| Argument       | Short | Description                                              |
| -------------- | ----- | -------------------------------------------------------- |
| `--session-id` | `-i`  | The session ID used to identify which migration to undo. |

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

You can change the path where `.messy-folder-reorganizer-ai` will be created. Simply add `MESSY_FOLDER_REORGANIZER_AI_PATH` environment variable with path with desired location.

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

### Additional help

- [Ollama GitHub](https://github.com/ollama/ollama)
- [Embedding Models with Ollama](https://ollama.com/blog/embedding-models)
- [Qdrant Docs](https://qdrant.tech/documentation/guides/installation/)

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

### Running tests:

To run all tests

```sh
cargo test
```

To run integration tests

```sh
cargo test --test '*' -- --nocapture
```

To run specific integration test (file_collision for example)

```sh
cargo test file_collision -- --nocapture
```

## Uninstall & Purge

```sh
rm -f /usr/local/bin/messy-folder-reorganizer-ai
rm -rf ~/.messy-folder-reorganizer-ai
```

## License

This project is dual-licensed under either:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

at your option.

It interacts with external services including:

- [Ollama](https://github.com/ollama/ollama) – MIT License
- [Qdrant](https://github.com/qdrant/qdrant) – Apache 2.0 License
