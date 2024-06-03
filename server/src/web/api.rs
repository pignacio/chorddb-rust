use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResult<T> {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<T>,
}

#[allow(dead_code)]
impl<T> ApiResult<T> {
    pub fn success(result: T) -> ApiResult<T> {
        ApiResult {
            success: true,
            message: None,
            result: Some(result),
        }
    }

    pub fn failure<S: AsRef<str>>(message: S) -> ApiResult<T> {
        ApiResult {
            success: false,
            message: Some(message.as_ref().to_string()),
            result: None,
        }
    }
}

pub type SimpleApiResult = ApiResult<()>;

impl SimpleApiResult {
    pub fn simple_success<S: AsRef<str>>(message: S) -> SimpleApiResult {
        ApiResult {
            success: true,
            message: Some(message.as_ref().to_string()),
            result: None,
        }
    }
}
