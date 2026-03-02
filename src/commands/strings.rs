use crate::{
    resp::RESP,
    storage_result::StorageResult,
    storage::Storage,
    error::Error,
};

pub struct SetCommand {
    pub key: String,
    pub value: String,
}

impl Command for SetCommand {
    fn execute(&self, args: &[String]) -> StorageResult<RESP> {
        match args {
            [key, value] => {
                store.set(key.to_string(), value.as_bytes().to_vec());
                Ok(RESP::SimpleString("OK".to_string()))
            }
            _ => Err(StorageError::CommandSyntaxError("SET requires 2 arguments".into())),
        }
    }
}

pub struct GetCommand {
    pub key: String,
}

impl Command for GetCommand {
    fn execute(&self, args: &[String]) -> StorageResult<RESP> {
        match args {
            [key] => match store.get(key) {
                Some(bytes) => Ok(RESP::BulkString(String::from_utf8(bytes).unwrap())),
                None => Ok(RESP::Null),
            }
            _ => Err(StorageError::CommandSyntaxError("GET requires 1 argument".into())),
        }
    }
}
