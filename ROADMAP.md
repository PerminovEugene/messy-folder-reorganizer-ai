# messy-folder-reorganizer-ai Roadmap

Current version: 0.2.0

Goal: Reach version 1.0.0 with a stable, user-friendly, and powerful AI-driven folder reorganization tool.

---

## Version 0.3.0 – Core Improvements & Stability

- [x] Add license
- [x] Review codebase and refactor if needed
- [x] Replace all `unwrap()`s with proper error handling using `Result` and custom error types
- [x] Improve error messages to be more informative and user-friendly
- [x] Handle filename collisions (e.g., `file (1).txt`)
- [x] Add rollback system via `plan.json` (reverse plan executor)
- [ ] Add unit and integration tests for:
  - Files parsing
  - File clustering logic
  - Plan generation
  - File movement
- [x] Check what's up with symlinks

---

### Tech decisions

#### Cleaning command

- Add command for cleaning collections? Currently there is a problem - for supporting multifolder processing - each collection keeps path segments, and isn't resetting before/after launching process. May be we need to remember new entities IDS and clean them up after CLI has finished

#### Migrations

- Each cli process command creates session_id. It's added to qdrant records, migration plan.
  If dirty flag is not provided then db session entities should stay, otherwise they should be deleted after process. Revert and apply now will expect migration plan file name.
  Some additional info about migrations should be saved for better UX. Currently Session ID is shown in logs and should be used for rollback and apply

#### Symlinks review:

- Symlinks become broken if file was moved, even if symlinks are in source dir they will be ignored and not updated by CLI. OS doesn't provide symlinks tracking for files, so it's on USER shoulders to support them, currently CLI shouldn't scan entire FS to track and update symlinks.

---

## Version 0.4.0 – One-Button Setup (Beginner Friendly)

- [ ] Add automatic installation script for:
  - Ollama (OS detection + install)
  - Required model downloads (`llama2`, `nomic-embed-text`)
  - Start Ollama server if not running
- [ ] Auto-setup Quadrant vector DB:
  - Docker or local binary
  - Create and persist workspace if not exists
- [ ] Add a README section: "One-command setup"
- [ ] Add `--log` option to save logs to file
- [ ] Add config file support for embedding logic

---

## Version 0.5.0 – Embedding Improvements

- [ ] Add support for embedding file contents (text-based only)
- [ ] Add embeddings context configuration
- [ ] Optimize LLM folder naming generation:
  - Use prompt templates that skip reasoning
  - Limit max tokens
  - Lower temperature
  - Trim output to first line
  - Set better default request parameters

---

## Version 0.6.0 – UX & CLI Enhancements

- [ ] Add logging and verbose mode
- [ ] Improve CLI UX:
  - Display cluster structure as tree
  - Better formatting and readability
- [ ] Support images parsing for abstract file names:
  - Detect images with unclear names (e.g. `IMG_1234.jpg`)
  - Use folder/cluster context to guess semantic meaning
  - Prepare logic for later image-based embedding support

---

## Version 0.7.0 – Showcase & Trust

- [ ] Record `.gif` and `.mp4` terminal demo of:
  - Scanning
  - Clustering
  - Confirmation
  - File movement
- [ ] Embed demo into README
- [ ] Add github fancy things to README.md

---

## Version 0.8.0 – Performance & Scale

- [ ] Optimize for large folders (>100000 files)
- [ ] Parallelize I/O and vector operations where appropriate
- [ ] Benchmark clustering and embedding speed

---

## Version 0.9.0 – Configurability & Plugins

- [ ] Pluggable backend model config (Ollama/OpenAI/custom)
- [ ] Add plugin system for pre-processing or context enrichment
- [ ] Support Windows OS

---

## Version 1.0.0 – Stable Public Release

- [ ] Final documentation and guides
- [ ] Add changelog and semver policy
- [ ] Try to add to brew
