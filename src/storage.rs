use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use crate::resp::RESP;
use crate::storage_result::{StorageError, StorageResult};

#[derive(Debug, PartialEq)]
pub enum StorageValue {
    String(String),
}

#[derive(Debug)]
pub struct StorageData {
    pub value: StorageValue,
    pub creation_time: SystemTime,
    pub expiry: Option<Duration>,
}

impl PartialEq for StorageData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub struct Storage {
    store: HashMap<String, StorageData>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    pub fn process_command(&mut self, command: &[String]) -> StorageResult<RESP> {
        if command.is_empty() {
            return Err(StorageError::IncorrectRequest);
        }
        match command[0].to_lowercase().as_str() {
            "ping" => self.command_ping(command),
            "echo" => self.command_echo(command),
            "set" => self.command_set(command),
            "get" => self.command_get(command),
            _ => Err(StorageError::CommandNotAvailable(command[0].clone())),
        }
    }

    fn command_ping(&self, _command: &[String]) -> StorageResult<RESP> {
        Ok(RESP::SimpleString("PONG".to_string()))
    }

    fn command_echo(&self, command: &[String]) -> StorageResult<RESP> {
        if command.len() < 2 {
            return Err(StorageError::CommandSyntaxError("ECHO requires 1 argument".into()));
        }
        Ok(RESP::BulkString(command[1].clone()))
    }

    fn command_set(&mut self, command: &[String]) -> StorageResult<RESP> {
        if command.len() < 3 {
            return Err(StorageError::CommandSyntaxError("SET requires 2 arguments".into()));
        }
        let key = command[1].clone();
        let value = command[2].clone();
        self.store.insert(
            key,
            StorageData {
                value: StorageValue::String(value),
                creation_time: SystemTime::now(),
                expiry: None,
            },
        );
        Ok(RESP::SimpleString("OK".to_string()))
    }

    fn command_get(&self, command: &[String]) -> StorageResult<RESP> {
        if command.len() < 2 {
            return Err(StorageError::CommandSyntaxError("GET requires 1 argument".into()));
        }
        let key = &command[1];
        match self.store.get(key) {
            Some(data) => match &data.value {
                StorageValue::String(s) => Ok(RESP::BulkString(s.clone())),
            },
            None => Ok(RESP::Null),
        }
    }
}

impl StorageData {
    pub fn add_expiry(&mut self, expiry: Duration) {
        self.expiry = Some(expiry);
    }
}

