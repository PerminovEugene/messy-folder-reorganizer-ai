# how to use test case

## Run before testing

```sh
  chflags uchg test_cases/failed_migration/messy-folder/Downloads/locked_doc.pdf
```

it will lock file only for migrating with fs::rename and CLI will generate failed migration log message. Folders structure parsing won't be affected

## Unlock after testing

```sh
  chflags nouchg test_cases/failed_migration/messy-folder/Downloads/locked_doc.pdf
```
