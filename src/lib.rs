use std::{collections::HashMap, fs::File, io::{self, ErrorKind, Read, Seek, Write}};
use std::io::Error as StdError;
use serde::{Deserialize, Serialize};


#[doc = r#"Trait necessary to push a doc to the database.
# Implementation
```
use memorable::MemoDoc;
struct Doc {
    uuid: String
}

impl MemoDoc for Doc {
    fn get_id(&self) -> &str {
        &self.uuid
    }

    fn set_id(&mut self, id: &str) {
        self.uuid = id.to_string();
    }
}
```
"#]
pub trait MemoDoc {
    fn get_id(&self) -> &str;
    fn set_id(&mut self, id: &str);
}

#[doc = r#"A wrapper over the file system to not expose the inner workings of file writing but
expose the `tasks vector` for eazy editability.
# Examples
```
use memorable::DataBase;
use memorable::memorable_macro_derive::MemoDoc;

#[derive(MemoDoc, Serialize, Deserialize)]
struct Task {
    uuid: String
}

fn main() {
    let f = DataBase::open("./db.json").unwrap();
    let tasks: Vec<Task> = f.docs;
}
```"#]
#[derive(Debug, Clone)]
pub struct DataBase<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> {
    file_path: String,
    pub docs: HashMap<String, T>
}

impl<T: Serialize + for<'de> Deserialize<'de> + MemoDoc + Clone> DataBase<T> {

#[doc = r#"Opens and fetches data from the `Tasks` database.

# Errors

This function may throw an `error` due to a number of different reasons. Some of them are listed below:
    1. Function will return an `io::error::Error` if there is any problem locating or opening the database's json file.
    2. Function will return an `serde_json::error:Error` if there is any problem serializing or de-serializing the data in the file.

# Examples
```
use memorable::DataBase;
fn main() {
    let f = DataBase::open("./path.json").unwrap();
    println!("Tasks: {:#?}", f.docs);
}
```"#]
    pub fn open(path: &str) -> Result<DataBase<T>, StdError> {
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
                f.read_to_string(&mut data)?;
                data
            },
            None => String::new()
        };

        let docs: HashMap<String, T> = match serde_json::from_str(&buff) {
            Ok(t) => t,
            Err(e) => {
                println!("Err: {e}");
                let op: HashMap<String, T> = HashMap::<String, T>::new();
                File::create(path)?.write_all(
                    serde_json::to_string_pretty(&op)?.as_bytes()
                )?;
                op
            },
        };

        Ok(Self {
            file_path: path.to_string(),
            docs
        })
    }

#[doc = r#"Adds a data to the database.

# Errors

This function may throw an `error` due to a number of different reasons. Some of them are listed bellow:
    1. Function will throw an `io::error::Error` if there is any problem locating or opening the database's json file.
    2. Function will throw an `serde_json::error:Error` if there is any problem serializing or de-serializing the data in the file.
    3. Function will throw an `io:error:Error` is input `data.get_id()` already exists in the data_base.
# Examples
```
use serde::Serialize;
use serde::Deserialize;
use memorable::DataBase;
use memorable_macro_derive::MemoDoc;
use memorable::MemoDoc;

#[derive(MemoDoc, Serialize, Deserialize, Default)]
struct Data {
    uuid: String,
    // other fields.
}

fn main() {
    let data = Data::default();
    let mut f = DataBase::open("./path.json").unwrap();
    f.push(data).unwrap();
    println!("{:#?}", f.datas);
}
```"#]
    pub fn push(&mut self, mut data: T) -> io::Result<()>{
        match self.docs.get(data.get_id()) {
            Some(_) => {
                return Err(StdError::new(ErrorKind::AlreadyExists, "data already exists"));
            },
            None => {}
        }
        if data.get_id() == "" {
            data.set_id(&uuid::Uuid::new_v4().to_string());
        }
        let mut file: File = File::options().truncate(false).read(true).write(true).open(&self.file_path)?;
        let mut buff: String = String::new();
        file.read_to_string(&mut buff)?;
        let mut docs: HashMap<String, T> = serde_json::from_str(&buff).unwrap_or_else(|e| {
            println!("Err: {}", e);
            HashMap::<String, T>::new()
        });
        docs.insert(data.get_id().to_string(), data);
        file.rewind()?;
        file.write_all(serde_json::to_string_pretty(&docs)?.as_bytes())?;
        self.docs = docs;
        Ok(())
    }

#[doc = r#"Deletes a data to the database.

# Errors

Function will throw an `io::error::Error` if no data was found with specified id.
# Examples
```
use serde::Serialize;
use serde::Deserialize;
use memorable::DataBase;
use memorable::memorable_macro_derive::MemoDoc;

#[derive(MemoDoc, Serialize, Deserialize, Default)]
struct Data {
    uuid: String,
    // other fields.
}

fn main() {
    let data = Data::default();
    let mut f = DataBase::open("./path.json");
    f.push(data).unwrap();
    let val = f.del(data.get_id()).unwrap();
    println!("Deleted: {:#?}", val);
    println!("Remaining: {:#?}", f.datas);
}
```"#]
    pub fn del(&mut self, id: &str) -> io::Result<T> {
        match self.docs.remove(id) {
            Some(v) => {
                let mut file: File = File::options().truncate(true).write(true).open(&self.file_path)?;
                let buff: String = serde_json::to_string_pretty(&self.docs)?;
                file.rewind()?;
                file.write_all(buff.as_bytes())?;
                Ok(v)
            },
            None => Err(StdError::new(ErrorKind::NotFound, format!("Data with specified ID ({id}) was not found.")))
        }
    }

#[doc = r#"Fetches a data to the database.

# Examples
```
use memorable::DataBase;
use memorable::memorable_macro_derive::MemoDoc;c

#[derive(MemoDoc, Serialize, Deserialize, Default)]
struct Data {
    uuid: String,
    // other fields.
}

fn main() {
    let data = Data::default();
    let mut f = DataBase::open("./path.json");
    f.push(data).unwrap();
    println!("Requested: {:#?}", f.get(data.get_id()));
}
```"#]
    pub fn get(&self, id: &str) -> Option<T> {
        self.docs.get(id).cloned()
    }
}
