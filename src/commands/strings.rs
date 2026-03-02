use crate::{
    resp::RESP,
    storage::Storage,
    storage_result::{StorageError, StorageResult},
};

pub trait Command {
    fn execute(&self, store: &mut Storage, args: &[String]) -> StorageResult<RESP>;
}

pub struct SetCommand;

impl Command for SetCommand {
    fn execute(&self, store: &mut Storage, args: &[String]) -> StorageResult<RESP> {
        match args {
            [key, value] => {
                store.process_command(&["SET".into(), key.clone(), value.clone()])
            }
            _ => Err(StorageError::CommandSyntaxError("SET requires 2 arguments".into())),
        }
    }
}

pub struct GetCommand;

impl Command for GetCommand {
    fn execute(&self, store: &mut Storage, args: &[String]) -> StorageResult<RESP> {
        match args {
            [key] => store.process_command(&["GET".into(), key.clone()]),
            _ => Err(StorageError::CommandSyntaxError("GET requires 1 argument".into())),
        }
    }
}