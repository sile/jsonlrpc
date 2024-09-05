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

#[cfg(test)]
mod tests {
    use std::net::{SocketAddr, TcpStream};

    use super::*;

    #[test]
    fn test_request() {
        let server_addr = spawn_server_thread();
        let socket = TcpStream::connect(server_addr).expect("failed to connect to server");
        let mut client = RpcClient::new(socket);

        let request = RequestObject {
            jsonrpc: JsonRpcVersion::V2,
            id: Some(RequestId::Number(1)),
            method: "foo".to_string(),
            params: None,
        };
        let Some(ResponseObject::Ok { result, id, .. }) =
            client.call(&request).expect("failed to send request")
        else {
            panic!("expected ok response, got notification or err response")
        };
        assert_eq!(id, RequestId::Number(1));
        assert_eq!(result, serde_json::Value::String("foo".to_string()));
    }

    #[test]
    fn test_notification() {
        let server_addr = spawn_server_thread();
        let socket = TcpStream::connect(server_addr).expect("failed to connect to server");
        let mut client = RpcClient::new(socket);

        let request = RequestObject {
            jsonrpc: JsonRpcVersion::V2,
            id: None,
            method: "foo".to_string(),
            params: None,
        };
        let response = client.call(&request).expect("failed to send request");
        assert!(response.is_none());
    }

    fn spawn_server_thread() -> SocketAddr {
        let listener =
            std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind to address");
        let addr = listener.local_addr().expect("failed to get local address");

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.expect("failed to accept incoming connection");
                let mut stream = JsonlStream::new(stream);
                std::thread::spawn(move || {
                    let request: RequestObject =
                        stream.read_item().expect("failed to read request");
                    if let Some(id) = request.id {
                        let response = ResponseObject::Ok {
                            jsonrpc: JsonRpcVersion::V2,
                            id,
                            result: serde_json::Value::String(request.method),
                        };
                        stream
                            .write_item(&response)
                            .expect("failed to write response");
                    }
                });
            }
        });

        addr
    }
}
