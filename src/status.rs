use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Idle,
    Running,
    Success,
    Failure,
    Error,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization() {
        {
            let json = serde_json::to_string(&Status::Idle {}).unwrap();
            assert!(r#""IDLE""# == json)
        }

        {
            let json = serde_json::to_string(&Status::Running {}).unwrap();
            assert!(r#""RUNNING""# == json)
        }

        {
            let json = serde_json::to_string(&Status::Success {}).unwrap();
            assert!(r#""SUCCESS""# == json)
        }

        {
            let json = serde_json::to_string(&Status::Failure {}).unwrap();
            assert!(r#""FAILURE""# == json)
        }

        {
            let json = serde_json::to_string(&Status::Error {}).unwrap();
            assert!(r#""ERROR""# == json)
        }
    }

    #[test]
    fn test_deserialization() {
        {
            let json = r#""IDLE""#;
            let status: Status = serde_json::from_str(json).unwrap();
            assert!(status == Status::Idle)
        }

        {
            let json = r#""RUNNING""#;
            let status: Status = serde_json::from_str(json).unwrap();
            assert!(status == Status::Running)
        }

        {
            let json = r#""SUCCESS""#;
            let status: Status = serde_json::from_str(json).unwrap();
            assert!(status == Status::Success)
        }

        {
            let json = r#""FAILURE""#;
            let status: Status = serde_json::from_str(json).unwrap();
            assert!(status == Status::Failure)
        }

        {
            let json = r#""ERROR""#;
            let status: Status = serde_json::from_str(json).unwrap();
            assert!(status == Status::Error)
        }
    }
}
