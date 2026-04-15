mod shutdown;

pub use shutdown::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Command {
    Shutdown(ShutdownRequest),
}

impl From<ShutdownRequest> for Command {
    fn from(value: ShutdownRequest) -> Self {
        Self::Shutdown(value)
    }
}
