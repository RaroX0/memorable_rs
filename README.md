# MEMORABLE

**MEMOry-duRABLE** is an experimental crate meant to create a high-level abstraction over Rust's file system and `serde_json` to store data into files in text JSON format durably.

## Features

- **Document Management**: Easily manage documents with the `MemoDoc` trait.
- **File-based Database**: Store and retrieve documents from a JSON file.
- **Error Handling**: Comprehensive error handling for file operations and JSON serialization/deserialization.
- **Derive Macro**: Automatically implement the `MemoDoc` trait for your structs.

## Usage

### Struct: `DataBase`
A wrapper over the file system to not expose the inner workings of file writing but exposes the docs vector for easy editability.

```rust
#[derive(Debug, Clone)]
pub struct DataBase<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> {
    file_path: String,
    pub docs: Vec<T>
}
```

### Trait: `MemoDoc`

The `MemoDoc` trait is necessary to push a document to the database.

```rust
pub trait MemoDoc {
    fn get_id(&self) -> &str;
    fn set_id(&mut self, id: &str);
}
```
### Derive Macro: `MemoDoc`
You can use the derive macro to implement the `MemoDoc` trait for your structs automatically.
```rust
#[derive(MemoDoc)]
pub struct MyDocument {
    uuid: String,
    // other fields
}
```

## Methods:

### `open`
Opens and fetches data from the `docs` database.
```rust
impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {
    pub fn open(path: &str) -> io::Result<DataBase<T>> {
        // Implementation
    }
}
```

### `push`
Adds a document to the database.
```rust
impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {
    pub fn push(&mut self, data: T) -> io::Result<()> {
        // Implementation
    }
}
```

### `del`
Deletes a document from the database.
```rust
impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {
    pub fn del(&mut self, id: &str) -> io::Result<T> {
        // Implementation
    }
}
```

### `get`
Fetches a document from the database.
```rust
impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {
    pub fn get(&self, id: &str) -> Option<T> {
        // Implementation
    }
}
```


## Example:

```rust
#[derive(MemoDoc, Serialize, Deserialize, Clone, Default)]
pub struct MyDocument {
    uuid: String,
    // other fields
}

fn main() {
    let db = DataBase::open("./path.json").unwrap();
    let data = MyDocument::default();
    db.push(data.clone()).unwrap();
    println!("Documents: {:?}", db.docs);

    let fetched_data = db.get(data.get_id()).unwrap();
    println!("Fetched: {:?}", fetched_data);

    let deleted_data = db.del(data.get_id()).unwrap();
    println!("Deleted: {:?}", deleted_data);
}
```
