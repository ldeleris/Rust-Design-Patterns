//! `proxy` module.
//!
//! # Example
//! 
//! ```rust,ignore
//! use structural::proxy::*;
//! use std::collections::HashMap;
//! 
//! let mut file_map: HashMap<&str, (&str, Box<FileReader>)> = HashMap::new();
//! file_map.insert("file1.txt", ("FileReaderProxy", Box::new(FileReaderProxy::new(String::from("file1.txt")))));
//! file_map.insert("file2.txt", ("FileReaderProxy", Box::new(FileReaderProxy::new(String::from("file2.txt")))));
//! file_map.insert("file3.txt", ("FileReaderProxy", Box::new(FileReaderProxy::new(String::from("file3.txt")))));
//! file_map.insert("file4.txt", ("FileReaderReal", Box::new(FileReaderReal::new(String::from("file1.txt")))));
//! 
//! println!("Created the map. You should have seen file1.txt read because it wasn't used in a proxy.");
//! for (k, v) in file_map.iter_mut() { 
//!     println!("Reading {} from the {}: {}", k, v.0, v.1.read_file_contents()); 
//! }
//! ```
  
use std::io::BufReader;
use std::fs::File;
use std::io::Read;

pub trait FileReader {
  fn read_file_contents(&mut self) -> String;
}

#[derive(Clone)]
pub struct FileReaderReal {
  file_name: String,
  contents: String,
}
impl FileReaderReal {
  pub fn new(file_name: String) -> FileReaderReal {
    println!("Finished reading the actual file: {}", file_name);
    FileReaderReal {
      contents: content(file_name.clone()),
      file_name,
    }
  }
}

fn content(path: String) -> String {
  let path = File::open(path).unwrap();
  let mut reader = BufReader::new(path);
  let mut buf = String::new();
  let _ = reader.read_to_string(&mut buf);
  buf
}

impl FileReader for FileReaderReal {
  fn read_file_contents(&mut self) -> String {
    self.contents.clone()
  }
}

pub struct FileReaderProxy {
  file_name: String,
  file_reader: Option<FileReaderReal>
}
impl FileReaderProxy {
  pub fn new(file_name: String) -> FileReaderProxy {
    FileReaderProxy {
      file_name,
      file_reader: None,
    }
  }
}
impl FileReader for FileReaderProxy {
  fn read_file_contents(&mut self) -> String {
    if let Some(ref mut file_reader) = self.file_reader {
      file_reader.read_file_contents()
    } else {
      let mut file_reader = FileReaderReal::new(self.file_name.clone());
      self.file_reader = Some(file_reader.clone());
      file_reader.read_file_contents()
    } 
  }
}