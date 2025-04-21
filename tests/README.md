## Integration Tests

Integration tests launch the CLI in a separate process.  
Each `test_case` contains a JSON file that defines the file system fixtures configuration.  
After launching the tests, this configuration is parsed, and new folders/files are created in the `test_cases/{test_case_name}/fixtures` directory. The CLI then uses these as source and destination paths.

Each test requires a CLI system folder. Currently, there are two options:

1. The test uses its own path and creates a system folder locally.
2. If the test requires deterministic LLM output, it uses the `tests/configs/` path, which includes system folder TOML configs that are not ignored by Git.

Each test also requires both Ollama and Qdrant to be running. Docker or CI/CD integration for these dependencies is not yet available but will be added later.

Each test case validates the `process`, `rollback`, and `apply` commands by asserting the updated file system fixtures.

Command outputs are saved in the following files:

- `process.log`
- `apply.log`
- `rollback.log`

These can be found under:  
`tests/test_cases/{test_case_name}/`
