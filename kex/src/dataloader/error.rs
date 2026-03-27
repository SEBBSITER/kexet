enum DataErrorType {
    PathMissing,
}

pub struct DataError {
    error: DataErrorType,
    message: String,
}

