use crate::error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShutdownRequest {
    /// The command completed without errors.
    Success,

    /// Catchall for miscellaneous errors (e.g., divide by zero).
    InternalError(String),

    /// Missing arguments or keyword/syntax errors.
    InputError(String),
}

impl ShutdownRequest {
    pub fn code(&self) -> u8 {
        match self {
            Self::Success => 0,
            Self::InternalError(_) => 1,
            Self::InputError(_) => 2,
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Success => None,
            Self::InternalError(m) => Some(m.as_str()),
            Self::InputError(m) => Some(m.as_str()),
        }
    }
}

impl std::fmt::Display for ShutdownRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.code())?;

        if let Some(m) = self.message() {
            write!(f, " - {}", m)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum Shutdown {
    Requested(ShutdownRequest),
    ReentryLimitExceeded,
    Error(error::Error),
}

impl Shutdown {
    pub fn code(&self) -> u8 {
        match self {
            Self::Requested(a) => a.code(),
            Self::ReentryLimitExceeded => 3,
            Self::Error(_) => 1,
        }
    }
}

impl std::process::Termination for Shutdown {
    fn report(self) -> std::process::ExitCode {
        std::process::ExitCode::from(self.code())
    }
}
