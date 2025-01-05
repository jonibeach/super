use std::collections::HashMap;

use crate::util::HttpInner;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum StatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    EarlyHints = 103,

    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    URITooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    MisdirectedRequest = 421,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    TooEarly = 425,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,

    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl StatusCode {
    fn to_string(&self) -> String {
        let str_repr = format!("{:?}", self);
        let str_repr: String = str_repr
            .char_indices()
            .flat_map(|(i, c)| {
                if c.is_uppercase() && i > 0 {
                    vec![' ', c]
                } else {
                    vec![c.to_ascii_uppercase()]
                }
            })
            .collect();

        format!("{} {str_repr}", *self as u16)
    }
}

pub trait IntoHttpRes<'a> {
    fn into_res(self) -> HttpRes<'a>;
}

impl<'a> IntoHttpRes<'a> for HttpRes<'a> {
    fn into_res(self) -> HttpRes<'a> {
        self
    }
}

impl<'a, T, E> IntoHttpRes<'a> for Result<T, E>
where
    T: IntoHttpRes<'a>,
    E: std::error::Error,
{
    fn into_res(self) -> HttpRes<'a> {
        match self {
            Ok(r) => r.into_res(),
            Err(e) => HttpRes::new(
                StatusCode::InternalServerError,
                Some(format!("Failed to process http request: {e}")),
            ),
        }
    }
}

#[derive(Debug)]
pub struct HttpRes<'a> {
    pub inner: HttpInner<'a>,
    pub status: StatusCode,
    pub body: Option<Vec<u8>>,
}

impl<'a> HttpRes<'a> {
    pub fn new(status: StatusCode, body: Option<impl Into<Vec<u8>>>) -> Self {
        let headers = HashMap::new();
        let major_ver = 1;
        let minor_ver = 1;

        let inner = HttpInner {
            major_ver,
            minor_ver,
            headers,
        };

        Self {
            status,
            inner,
            body: body.map(|b| b.into()),
        }
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(
            format!(
                "HTTP/{}.{} {}\r\n",
                self.inner.major_ver,
                self.inner.minor_ver,
                self.status.to_string()
            )
            .as_bytes(),
        );

        for (h, v) in &self.inner.headers {
            bytes.extend_from_slice(format!("{}: {}\r\n", h, v).as_bytes());
        }

        bytes.extend_from_slice(&[b'\r', b'\n']);

        if let Some(b) = &self.body {
            bytes.extend_from_slice(&b[..]);
        }

        bytes
    }
}
