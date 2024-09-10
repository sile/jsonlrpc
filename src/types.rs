use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

/// JSON-RPC version.
#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum JsonRpcVersion {
    /// JSON-RPC 2.0.
    #[default]
    #[serde(rename = "2.0")]
    V2,
}

impl FromStr for JsonRpcVersion {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for JsonRpcVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Request ID.
///
/// This representation does not accept `null` and floating-point numbers,
/// as these are discouraged by the specification.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestId {
    /// Numeric ID.
    Number(i64),

    /// String ID.
    String(String),
}

impl FromStr for RequestId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Request parameters.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestParams {
    /// By-position parameters.
    Array(Vec<serde_json::Value>),

    /// By-name parameters.
    Object(serde_json::Map<String, serde_json::Value>),
}

impl FromStr for RequestParams {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for RequestParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Default representation for a request object.
///
/// Note that this is merely a default representation.
/// Users are free to define their own request structures or enums as shown below:
///
/// ```
/// use serde::{Deserialize, Serialize};
/// use jsonlrpc::{JsonRpcVersion, RequestId};
///
/// #[derive(Serialize, Deserialize)]
/// #[serde(tag = "method", rename_all = "snake_case")]
/// enum KvsRequest {
///     Put { jsonrpc: JsonRpcVersion, id: RequestId, key: String, value: String },
///     Get { jsonrpc: JsonRpcVersion, id: RequestId, key: String },
///     Delete { jsonrpc: JsonRpcVersion, key: String },
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RequestObject {
    /// JSON-RPC version.
    pub jsonrpc: JsonRpcVersion,

    /// Method name.
    pub method: String,

    /// Request parameters.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<RequestParams>,

    /// Request ID.
    ///
    /// If `None`, the request is a notification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<RequestId>,
}

impl FromStr for RequestObject {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for RequestObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Single or batch object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeBatch<T> {
    /// Single object.
    Single(T),

    /// Batch object.
    Batch(Vec<T>),
}

impl<T> MaybeBatch<T> {
    /// Returns the number of objects in this instance.
    pub fn len(&self) -> usize {
        match self {
            MaybeBatch::Single(_) => 1,
            MaybeBatch::Batch(v) => v.len(),
        }
    }

    /// Returns `true` if this instance is a batch object.
    pub fn is_batch(&self) -> bool {
        matches!(self, MaybeBatch::Batch(_))
    }

    /// Returns an iterator over the objects in this instance.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            MaybeBatch::Single(v) => Some(v).into_iter().chain(None.into_iter().flatten()),
            MaybeBatch::Batch(v) => None.into_iter().chain(Some(v.iter()).into_iter().flatten()),
        }
    }

    /// Returns a mutable iterator over the objects in this instance.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        match self {
            MaybeBatch::Single(v) => Some(v).into_iter().chain(None.into_iter().flatten()),
            MaybeBatch::Batch(v) => None
                .into_iter()
                .chain(Some(v.iter_mut()).into_iter().flatten()),
        }
    }
}

impl<T> FromStr for MaybeBatch<T>
where
    T: for<'de> Deserialize<'de>,
{
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl<T> Display for MaybeBatch<T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Default representation for a response object.
///
/// Note that this is merely a default representation.
/// Users are free to use any structs or enums as long as they implement the [`Serialize`] and [`Deserialize`] traits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResponseObject {
    /// Success response.
    Ok {
        /// JSON-RPC version.
        jsonrpc: JsonRpcVersion,

        /// Result value.
        result: serde_json::Value,

        /// Request ID.
        id: RequestId,
    },

    /// Error response.
    Err {
        /// JSON-RPC version.
        jsonrpc: JsonRpcVersion,

        /// Error information.
        error: ErrorObject,

        /// Request ID.
        ///
        /// If deserialization of the associated request fails, this field will be `None`.
        id: Option<RequestId>,
    },
}

impl ResponseObject {
    /// Returns the request ID associated with this response.
    pub fn id(&self) -> Option<&RequestId> {
        match self {
            ResponseObject::Ok { id, .. } => Some(id),
            ResponseObject::Err { id, .. } => id.as_ref(),
        }
    }

    /// Returns `Ok(result)` if this response is a success response, otherwise `Err(error)`.
    pub fn to_std_result(&self) -> Result<&serde_json::Value, &ErrorObject> {
        match self {
            ResponseObject::Ok { result, .. } => Ok(result),
            ResponseObject::Err { error, .. } => Err(error),
        }
    }

    /// Converts this response object into a standard result.
    pub fn into_std_result(self) -> Result<serde_json::Value, ErrorObject> {
        match self {
            ResponseObject::Ok { result, .. } => Ok(result),
            ResponseObject::Err { error, .. } => Err(error),
        }
    }
}

impl FromStr for ResponseObject {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl Display for ResponseObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Error object.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ErrorObject {
    /// Error code.
    pub code: ErrorCode,

    /// Error message.
    pub message: String,

    /// Additional information about the error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl FromStr for ErrorObject {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for ErrorObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}

/// Error code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ErrorCode(i32);

impl ErrorCode {
    /// Parse error.
    pub const PARSE_ERROR: Self = Self(-32700);

    /// Invalid request.
    pub const INVALID_REQUEST: Self = Self(-32600);

    /// Method not found.
    pub const METHOD_NOT_FOUND: Self = Self(-32601);

    /// Invalid parameters.
    pub const INVALID_PARAMS: Self = Self(-32602);

    /// Internal error.
    pub const INTERNAL_ERROR: Self = Self(-32603);

    /// Makes a new [`ErrorCode`] instance.
    pub const fn new(code: i32) -> Self {
        Self(code)
    }

    /// Returns the error code value.
    pub const fn get(self) -> i32 {
        self.0
    }

    /// Returns `true` if the error code is a pre-defined error code.
    pub const fn is_pre_defined(self) -> bool {
        -32768 <= self.0 && self.0 <= -32000
    }

    /// Returns `true` if the error code is a server error.
    pub const fn is_server_error(self) -> bool {
        -32099 <= self.0 && self.0 <= -32000
    }

    /// A convenience method to guess the error code from a [`serde_json::Error`].
    pub fn guess(error: &serde_json::Error) -> Self {
        match error.classify() {
            serde_json::error::Category::Io => Self::INTERNAL_ERROR,
            serde_json::error::Category::Syntax => Self::PARSE_ERROR,
            serde_json::error::Category::Data => Self::INVALID_REQUEST,
            serde_json::error::Category::Eof => Self::PARSE_ERROR,
        }
    }
}

impl FromStr for ErrorCode {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        serde_json::to_string(self).expect("unreachable").fmt(f)
    }
}
