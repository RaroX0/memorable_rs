use std::{fs::File, io::{Error, Read, Seek, Write}};
use serde::{Deserialize, Serialize};


#[doc = r#"Trait necessary to push a doc to the database.
# Implementation
```
fn get_id(&self) -> &str {
    &self.uuid
}

fn set_id(&mut self, id: &str) {
    self.uuid = id.to_string();
}
"#]
pub trait MemoDoc {
    fn get_id(&self) -> &str;
    fn set_id(&mut self, id: &str);
}

#[doc = r#"A wrapper over the file system to not expose the inner workings of file writing but
expose the `tasks vector` for eazy editability.
# Examples
```
let f = DataBase::open("./db.json"); /// Returns a loaded instance of `DataBase`.
let tasks: Vec<Task> = f.tasks; /// List of added Tasks.
```"#]
pub struct DataBase<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> {
    file_path: String,
    pub docs: Vec<T>
}

impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {

#[doc = r#"Opens and fetches data from the `Tasks` database.
# Examples
```
fn main() {
    let f = DataBase::open("./path.json");
    println!("Tasks: {#:?}", f.tasks);
}
```"#]
    pub fn open(path: &str) -> DataBase<T> {
        let file: Option<File> = match File::open(path) {
            Ok(f) => Some(f),
            Err(e) => {
                println!("Err: {e}");
                None
            }
        };

        let buff: String = match file {
            Some(mut f) => {
                let mut data: String = String::new();
                f.read_to_string(&mut data).unwrap();
                data
            },
            None => String::new()
        };

        let docs: Vec<T> = match serde_json::from_str(&buff) {
            Ok(t) => t,
            Err(e) => {
                println!("Err: {e}");
                let op: Vec<T> = Vec::new();
                File::create(path).unwrap().write_all(
                    serde_json::to_string_pretty(&op).unwrap().as_bytes()
                ).unwrap();
                op
            },
        };

        Self {
            file_path: path.to_string(),
            docs
        }
    }

#[doc = r#"Adds a data to the database.

# Errors

This function may throw an `error` due to a number of different reasons. Some of them are listed bellow:
    1. Function will throw an `io::error::Error` if there is any problem locating or opening the database's json file.
    2. Function will throw an `serde_json::error:Error` if there is any problem serializing or de-serializing the data in the file.
# Examples
```
fn main() {
    let data = Data::default();
    let mut f = DataBase::open("./path.json");
    f.push(data).unwrap();
    println!("{:#?}", f.datas);
}
```"#]
    pub fn push(&mut self, data: T) -> Result<(), Error>{
        let mut file: File = File::options().truncate(false).read(true).write(true).open(&self.file_path)?;
        let mut buff: String = String::new();
        file.read_to_string(&mut buff)?;
        let mut docs: Vec<T> = serde_json::from_str(&buff).unwrap_or_else(|e| {
            println!("Err: {}", e);
            Vec::<T>::new()
        });
        docs.push(data);
        file.rewind()?;
        file.write_all(serde_json::to_string_pretty(&docs)?.as_bytes())?;
        self.docs = docs;
        Ok(())
    }
}
