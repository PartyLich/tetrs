#[derive(Debug)]
pub enum Error {
    // MissingComponent(String),
    MissingComponent(&'static str),
    ExternalString(String),
    // External(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::MissingComponent(name) => write!(f, "Component not found: {}", name),
            Self::ExternalString(ref e) => write!(f, "{}", e),
            // Self::External(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            // Self::External(ref e) => Some(e),
            // Generic error, underlying cause isn't tracked.
            _ => None,
        }
    }
}

impl From<String> for Error {
    fn from(err_str: String) -> Self {
        Self::ExternalString(err_str)
    }
}
