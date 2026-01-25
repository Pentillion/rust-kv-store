use std::{collections::HashMap, fs::{File, OpenOptions}, io::{BufReader, Write}, sync::Mutex};
use std::io::{BufRead};

pub struct Wal {
    file: Mutex<File>,
    path: String
}

pub enum WalEntry {
    Set(String, String),
    Delete(String),
    Clear,
}

#[derive(Debug)]
pub enum WalError {
    PoisonedLock,
    IoError(std::io::Error),
}

impl WalEntry {
    pub fn to_line(&self) -> String {
        match self {
            WalEntry::Set(k, v) => format!("SET {} {}\n", k, v),
            WalEntry::Delete(k) => format!("DELETE {}\n", k),
            WalEntry::Clear => "CLEAR\n".to_string()
        }
    }

    pub fn from_line(line: &str) -> Result<Self, String> {
        let mut parts = line.splitn(3, ' ');
        match parts.next() {
            Some("SET") => {
                let key = parts.next().ok_or("Missing key in SET")?.to_string();
                let value = parts.next().ok_or("Missing value in SET")?.to_string();
                Ok(WalEntry::Set(key, value))
            }
            Some("DELETE") => {
                let key = parts.next().ok_or("Missing key in DELETE")?.to_string();
                Ok(WalEntry::Delete(key))
            }
            Some("CLEAR") => Ok(WalEntry::Clear),
            Some(cmd) => Err(format!("Unknown WAL command: {}", cmd)),
            None => Err("Empty WAL line".to_string()),
        }
    }
}

impl Wal {
    pub fn open(path: &str) -> Result<Self, String> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .map_err(|e| format!("Failed to open WAL file: {}", e))?;

        Ok(Wal {
            file: Mutex::new(file),
            path: path.to_string()
        })
    }
    
    pub fn append(&self, entry: WalEntry) -> Result<(), WalError> {
        let mut file = self.file.lock().map_err(|_| WalError::PoisonedLock)?;
        file.write_all(entry.to_line().as_bytes()).map_err(WalError::IoError)?;
        file.flush().map_err(WalError::IoError)?;
        Ok(())
    }

    pub fn replay(&self, map: &mut HashMap<String, String>) -> Result<(), String> {
        let file = File::open(&self.path)
            .map_err(|e| format!("Failed to open WAL for replay: {}", e))?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let entry = WalEntry::from_line(&line)?;
            
            match entry {
                WalEntry::Set(k, v) => { map.insert(k, v); }
                WalEntry::Delete(k) => { map.remove(&k); }
                WalEntry::Clear => { map.clear(); }
            }
        }

        Ok(())
    }

    pub fn truncate(&self) -> Result<(), String> {
        let _lock = self.file.lock().unwrap();

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.path)
            .map_err(|e| format!("Failed to truncate WAL: {}", e))?;

        f.flush().map_err(|e| format!("Failed to flush WAL: {}", e))?;
        Ok(())
    }
}