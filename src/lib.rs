mod io;
mod types;

pub use io::JsonlStream;
use serde::{Deserialize, Serialize};
pub use types::{
    ErrorCode, ErrorObject, JsonRpcVersion, RequestId, RequestObject, RequestParams, ResponseObject,
};

pub trait Request: Serialize + for<'a> Deserialize<'a> {
    type Response: Serialize + for<'a> Deserialize<'a>;

    fn is_notification(&self) -> bool;
}
