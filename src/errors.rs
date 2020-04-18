use actix_web::error;
use failure::Fail;

#[derive(Fail, Debug)]
#[fail(display = "connection error")]
pub struct ConnectionError {
    pub conn_name: &'static str,
}
// Use default implementation for `error_response()` method
impl error::ResponseError for ConnectionError {}