use serde::{Deserialize, Serialize};

mod io;
mod rpc;
mod types;

pub use io::JsonlStream;
pub use rpc::RpcClient;
pub use types::{
    ErrorCode, ErrorObject, JsonRpcVersion, RequestId, RequestObject, RequestParams, ResponseObject,
};

pub trait Request: Serialize + for<'a> Deserialize<'a> {
    type Response: Serialize + for<'a> Deserialize<'a>;

    fn is_notification(&self) -> bool;
}

impl Request for RequestObject {
    type Response = ResponseObject;

    fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}
