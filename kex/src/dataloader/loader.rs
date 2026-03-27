use std::path::PathBuf;
use crate::dataloader::error::DataError;

pub struct DataLoader {
    data_dir: PathBuf,
}

impl DataLoader {
    pub fn new(data_dir: PathBuf) -> DataLoader {
        Self {
            data_dir,
        }
    }

    pub fn partition_data() -> Result<(), DataError> {
        Ok(())
    }
}