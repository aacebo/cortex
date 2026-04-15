#[derive(Debug)]
pub enum LayerError {
    Message(String),
    Other(Box<dyn std::error::Error>),
}

impl std::fmt::Display for LayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(v) => write!(f, "{}", v),
            Self::Other(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for LayerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Other(v) => Some(v.as_ref()),
            _ => None,
        }
    }
}
