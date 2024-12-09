use orthrus_core::data::DataError;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum FerroxError {
    #[snafu(display("Error when reading/writing a data stream: {source}"))]
    DataError { source: DataError },
}

impl From<DataError> for FerroxError {
    fn from(source: DataError) -> Self {
        FerroxError::DataError { source }
    }
}
