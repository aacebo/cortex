mod layer;

pub use layer::*;

#[derive(Debug)]
pub enum Error {
    Layer(LayerError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Layer(v) => write!(f, "{}", v),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Layer(v) => v.source(),
        }
    }
}
