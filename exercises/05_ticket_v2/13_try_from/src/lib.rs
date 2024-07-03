// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("foo")]
struct StatusParsingError;

impl TryFrom<String> for Status {
    type Error = StatusParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value: String = value.to_lowercase();
        if value == "todo"{
            Ok(Status::ToDo)
        } else if value == "inprogress"{
            Ok(Status::InProgress)
        } else if value == "done" {
            Ok(Status::Done)
        } else {
            Err(StatusParsingError)
        }
    }
}
impl TryFrom<&str> for Status {
    type Error = StatusParsingError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: String = (*value).to_lowercase();
        if value == "todo"{
            Ok(Status::ToDo)
        } else if value == "inprogress"{
            Ok(Status::InProgress)
        } else if value == "done" {
            Ok(Status::Done)
        } else {
            Err(StatusParsingError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);

        let status = Status::try_from("unkownwnwnnw".to_string()).unwrap_err();
        assert_eq!(status, StatusParsingError);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
