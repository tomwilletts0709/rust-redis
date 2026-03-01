use crate::resp::RESP;
use crate::storage_result::{StorageError, StorageResult}
use std::colelctins::HashMap;

#[derive(Debugm PartialEq)]
pub enum StorageValue {
    String(String),
}

pub struct Storage {
    store: HashMap<String, StorageData>,
}

impl Storage {
    pub fn new() -> Self {
        let store: HashMap<String, StorageValue> = HashMap::new();

        Self { store: store}
    }

    pub fn process_command(&mut self, command: &Vec<String>) -> StorageResult<RESP> {
        match command[0].to_lowercase().as_str() {
            "ping" => self.command_ping(&command),
            "echo" => self.command_echo(&command),
            "set" => self.command_set(&command),
            "get" => self.command_get(&command),
            _ => Err(StorageError::CommandNotAvailable(command[0].clone()))
        }
    }
    
    fn command_ping(&self, _command: &Vec<String>) -> StorageResult<RESP> {
        Ok(RESP::SimpleString("PONG".to_string()))
    }
    
    fn command_echo(&self, command: &Vec<String>) -> StorageResult<RESP> {
        Ok(RESP::BulkString(command[1].clone()))
    }
}

impl StorageData {
    pub fn add_expiry(&mut self, expiry: Duration) {
        self.expiry = Some(expiry);
    }
}

impl From<String> for StorageData {
    fn from(s: String) -> StorageData {
        StorageData {
            value: StorageValue::String(s),
            creation_time: SystemTime::now(),
            expiry: None,
        }
    }
}

impl PartialEq for StorageData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

