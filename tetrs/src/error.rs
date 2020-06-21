use ecs::error::Error as EcsError;

#[derive(Debug)]
pub enum Error {
    MissingComponent(&'static str),
    NoCurrentPiece,
    ExternalString(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::MissingComponent(name) => write!(f, "Component not found: {}", name),
            Self::NoCurrentPiece => write!(f, "There's no current game piece!"),
            Self::ExternalString(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<String> for Error {
    fn from(err_str: String) -> Self {
        Self::ExternalString(err_str)
    }
}

impl From<EcsError> for Error {
    fn from(err: EcsError) -> Self {
        match err {
            EcsError::MissingComponent(val) => Self::MissingComponent(val),
            EcsError::ExternalString(val) => Self::ExternalString(val),
        }
    }
}
