# MEMOry-duRABLE

**MEMOry-duRABLE** is an experimental crate meant to create a high-level abstraction over Rust's file system and `serde_json` to durably store data into files in text JSON format.

## Features

- **Document Management**: Easily manage documents with the `MemoDoc` trait.
- **File-based Database**: Store and retrieve documents from a JSON file.
- **Error Handling**: Comprehensive error handling for file operations and JSON serialization/deserialization.
- **Derive Macro**: Automatically implement the `MemoDoc` trait for your structs.

## Usage

### Trait: `MemoDoc`

The `MemoDoc` trait is necessary to push a document to the database.

```rust
pub trait MemoDoc {
    fn get_id(&self) -> &str;
    fn set_id(&mut self, id: &str);
}
```
