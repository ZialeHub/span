pub mod date;
pub mod datetime;
pub mod time;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum TimeRSError {
    #[error("{0} ➤  Parse: {1}")]
    Parse(Type, String),
    #[error("{0} ➤  InvalidUpdate: {1}")]
    InvalidUpdate(Type, String),
}

#[derive(thiserror::Error, Debug, derive_more::Display, PartialEq)]
pub enum Type {
    Datetime,
    Date,
    Time,
}

pub type TimeResult<T> = Result<T, TimeRSError>;

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_log_error_datetime_parse() {
        let err = TimeRSError::Parse(Type::Datetime, "parse_from_str error".to_string());
        assert_eq!(err.to_string(), "Datetime ➤  Parse: parse_from_str error")
    }

    #[test]
    fn test_log_error_datetime_invalid_update() {
        let err = TimeRSError::InvalidUpdate(
            Type::Datetime,
            "Cannot add x Day to datetime error".to_string(),
        );
        assert_eq!(
            err.to_string(),
            "Datetime ➤  InvalidUpdate: Cannot add x Day to datetime error"
        )
    }

    #[test]
    fn test_log_error_date_parse() {
        let err = TimeRSError::Parse(Type::Date, "parse_from_str error".to_string());
        assert_eq!(err.to_string(), "Date ➤  Parse: parse_from_str error")
    }

    #[test]
    fn test_log_error_date_invalid_update() {
        let err =
            TimeRSError::InvalidUpdate(Type::Date, "Cannot add x Month to date error".to_string());
        assert_eq!(
            err.to_string(),
            "Date ➤  InvalidUpdate: Cannot add x Month to date error"
        )
    }

    #[test]
    fn test_log_error_time_parse() {
        let err = TimeRSError::Parse(Type::Time, "parse_from_str error".to_string());
        assert_eq!(err.to_string(), "Time ➤  Parse: parse_from_str error")
    }

    #[test]
    fn test_log_error_time_invalid_update() {
        let err =
            TimeRSError::InvalidUpdate(Type::Time, "Cannot add x Second to time error".to_string());
        assert_eq!(
            err.to_string(),
            "Time ➤  InvalidUpdate: Cannot add x Second to time error"
        )
    }
}
