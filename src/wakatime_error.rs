use std::fmt;

#[derive(Debug)]
pub enum WakatimeApiError {
    RequestError(reqwest::Error),
    DeserializationError(std::io::Error),
}

impl fmt::Display for WakatimeApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WakatimeApiError::RequestError(e) => write!(f, "Erreur de requête: {}", e),
            WakatimeApiError::DeserializationError(e) => write!(f, "Erreur de désérialisation: {}", e),
        }
    }
}
impl From<reqwest::Error> for WakatimeApiError {
    fn from(err: reqwest::Error) -> Self {
        WakatimeApiError::RequestError(err)
    }
}

impl From<std::io::Error> for WakatimeApiError {
    fn from(err: std::io::Error) -> Self {
        WakatimeApiError::DeserializationError(err)
    }
}