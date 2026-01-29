# Dictionary Management Guide

This guide explains how to add, remove, or update words in the GoxViet English dictionary using the provided helper scripts.

The dictionary engine uses pre-compiled binary files (`.bin`) found in `core/src/engine_v2/english/data/` which are synchronized with human-readable text files in `.docs/features/core-engine/data/`.

## Architecture

The dictionary management system follows this workflow:

1. **Source of Truth**: `.docs/features/core-engine/data/*.txt` files contain all dictionary words in human-readable format.
2. **Whitelist/Blacklist**: `scripts/dict_config/whitelist.txt` and `blacklist.txt` control word inclusion/exclusion.
3. **Binary Generation**: `scripts/generate_binary_dict.py` reads the text files and generates optimized `.bin` files.
4. **Documentation Sync**: `scripts/decode_dictionary.py` ensures text files match binary files.

## 1. Helper Script

We provide a unified script `scripts/manage_dict.py` to handle all dictionary operations. Ensure you are in the project root directory when running these commands.

```bash
# General Usage
./scripts/manage_dict.py [COMMAND] [WORDS...]
```

## 2. Adding Words

To add one or more words to the dictionary (ensuring they are whitelisted and not filtered out):

```bash
# Add a single word
./scripts/manage_dict.py add hello

# Add multiple words
./scripts/manage_dict.py add world universe galaxy
```

This will:
1. Add the words to `scripts/dict_config/whitelist.txt`.
2. Regenerate the binary dictionary files.
3. Update the documentation files in `.docs/features/core-engine/data/`.

## 3. Removing Words

To remove words from the dictionary (blacklisting them so they are not included):

```bash
# Remove a single word
./scripts/manage_dict.py remove russ

# Remove multiple words
./scripts/manage_dict.py remove caa badword junk
```

This will:
1. Add the words to `scripts/dict_config/blacklist.txt`.
2. Remove them from `scripts/dict_config/whitelist.txt` (if present).
3. Regenerate the binary dictionary files.
4. Update the documentation files.

## 4. Updating Words

The `update` command is an alias for `add`. Use it to ensure a list of words is present in the dictionary.

```bash
./scripts/manage_dict.py update define example test
```

## 5. Manual Configuration

The script modifies the following configuration files:

- **Whitelist**: `scripts/dict_config/whitelist.txt`
  - Words here are *always* included in the dictionary, bypassing conflict checks and filters.
- **Blacklist**: `scripts/dict_config/blacklist.txt`
  - Words here are *never* included in the dictionary.

You can edit these files manually and then run the sync command:

```bash
./scripts/manage_dict.py sync
```

## 6. Reverse Sync (Docs -> Binaries)

If you manually edit the text files in `.docs/features/core-engine/data/`, you can push those changes back to the binary files using:

```bash
./scripts/manage_dict.py reverse-sync
```

**Note:** This is a manual override. If you run `add`, `remove`, or `sync` later, the binaries *will be overwritten* by the standard generation process (which uses the source data + whitelist). To persist changes permanently, usage of `add`/`remove` commands (whitelisting/blacklisting) is recommended.

## 7. Generated Files

The process updates:
- **Binaries**: `core/src/engine_v2/english/data/*.bin` (Used by the Rust Engine)
- **Documentation**: `.docs/features/core-engine/data/*.txt` (For human reference)
