#[derive(Debug)]
pub enum Error {
    DuplicateNode(String),
    MissingNode(String),
    Cycle(String),
    UnknownLeaf(String),
    DuplicateLeaf(String),
    InvalidConfig { node: String, reason: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        {
            let err = Error::DuplicateNode("".into());
            eprintln!("Error: {}", err);
        }

        {
            let err = Error::MissingNode("".into());
            eprintln!("Error: {}", err);
        }

        {
            let err = Error::Cycle("".into());
            eprintln!("Error: {}", err);
        }

        {
            let err = Error::UnknownLeaf("".into());
            eprintln!("Error: {}", err);
        }

        {
            let err = Error::DuplicateLeaf("".into());
            eprintln!("Error: {}", err);
        }

        {
            let err = Error::InvalidConfig {
                node: "".into(),
                reason: "".into(),
            };
            eprintln!("Error: {}", err);
        }
    }
}
