use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, LineWriter, Write},
    path::Path,
};

use crate::errors::{KvsError, Result};

/// a diy kv store
pub struct KvStore {
    store: HashMap<String, String>,
    wal: File,
}

#[derive(Serialize, Deserialize)]
enum Op {
    Set,
    Get,
    Remove,
}

#[derive(Serialize, Deserialize)]
struct LogEntry {
    op: Op,
    key: String,
    val: Option<String>,
}

impl KvStore {
    /// create a kv store
    pub fn new(log_file: File) -> KvStore {
        KvStore {
            store: HashMap::new(),
            wal: log_file,
        }
    }
    /// set kv pair
    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        self.write_log(&LogEntry {
            op: Op::Set,
            key: key.clone(),
            val: Some(value.clone()),
        })?;
        self.store.insert(key, value);
        Ok(())
    }
    /// get value by key
    pub fn get(&self, key: String) -> Result<Option<String>> {
        // get req has no need to write wal
        let res = self.store.get(&key).cloned();
        Ok(res)
    }
    /// remove kv pair by key
    pub fn remove(&mut self, key: String) -> Result<()> {
        if !self.store.contains_key(&key) {
            return Err(KvsError::KeyNotFound);
        }
        self.write_log(&LogEntry {
            op: Op::Remove,
            key: key.clone(),
            val: None,
        })?;
        self.store.remove(&key);
        Ok(())
    }
    /// open kvstore on a specific path
    pub fn open(path: &Path) -> Result<KvStore> {
        // create_dir_all(path)?;
        let file_path = path.join("wal");
        let file_res = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .append(true)
            .open(file_path);
        if file_res.is_err() {
            return Err(KvsError::LogFileIOError {
                msg: file_res.unwrap_err().to_string(),
            });
        }
        let mut f = file_res.unwrap();
        let buffered = BufReader::new(&mut f);
        let mut inner_store: HashMap<String, String> = HashMap::new();
        for line in buffered.lines() {
            match line {
                Ok(line) => {
                    let log_entry = serde_json::from_str(&line);
                    match log_entry {
                        Err(e) => {
                            return Err(KvsError::SerializationError { msg: e.to_string() });
                        }
                        Ok(log_entry) => {
                            replay(&mut inner_store, log_entry);
                        }
                    }
                }
                Err(e) => return Err(KvsError::SerializationError { msg: e.to_string() }),
            }
        }
        Ok(KvStore {
            store: inner_store,
            wal: f,
        })
    }

    fn write_log(&mut self, cmd: &LogEntry) -> Result<()> {
        let buf = serde_json::to_vec(cmd);
        if buf.is_err() {
            return Err(KvsError::SerializationError {
                msg: buf.unwrap_err().to_string(),
            });
        }
        let mut writer = LineWriter::new(&mut self.wal);
        match writer.write(&buf.unwrap()) {
            Ok(_bytes) => {
                writer.write(b"\n")?;
                Ok(())
            }
            Err(e) => Err(KvsError::LogFileIOError { msg: e.to_string() }),
        }
    }
    /// util for debug
    pub fn ls(&self) {
        for (k, v) in &self.store {
            println!("{} - {}", k, v);
        }
    }
}

fn replay(store: &mut HashMap<String, String>, log: LogEntry) {
    match log.op {
        Op::Set => {
            store.insert(log.key, log.val.unwrap_or_default());
        }
        Op::Remove => {
            store.remove(&log.key);
        }
        _ => {}
    }
}
