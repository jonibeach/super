use std::collections::HashMap;

use anyhow::{anyhow, Context};

macro_rules! match_or_err {
    ($val: expr, $should_match: expr) => {
        match $val {
            $should_match => {}
            n => {
                return Err(anyhow::anyhow!(
                    "Expected '{:?}', got '{n:?}'",
                    $should_match
                ))
            }
        }
    };
}

#[derive(Debug)]
pub struct HttpInner<'a> {
    pub major_ver: usize,
    pub minor_ver: usize,
    pub headers: HashMap<&'a str, &'a str>,
}

impl<'a> HttpInner<'a> {
    pub fn from_byte_lines(lines: &Vec<&'a [u8]>) -> anyhow::Result<Self> {
        let mut ver_line = lines[0];
        match_or_err!(&ver_line[..5], b"HTTP/");
        ver_line = &ver_line[5..];

        let sep_idx = ver_line
            .iter()
            .enumerate()
            .find_map(|(i, b)| if *b == b'.' { Some(i) } else { None })
            .context("Version string doesn't contain '.'")?;

        let (major_bytes, minor_bytes) = ver_line.split_at(sep_idx);
        let (major_str, minor_str) = (
            core::str::from_utf8(major_bytes).context(format!(
                "Cannot parse major version from bytes {major_bytes:?}"
            ))?,
            core::str::from_utf8(&minor_bytes[1..]).context(format!(
                "Cannot parse minor version from bytes {:?}",
                &minor_bytes[1..]
            ))?,
        );

        let (major_str, minor_str) = (major_str.trim(), minor_str.trim());

        let (major_ver, minor_ver) = (
            major_str
                .parse()
                .context(format!("Cannot parse major version from str '{major_str}'"))?,
            minor_str
                .parse()
                .context(format!("Cannot parse minor version from str '{minor_str}'"))?,
        );

        let mut headers = HashMap::new();

        for l in &lines[1..lines.len() - 2] {
            let str = core::str::from_utf8(l).context(format!(
                "Header line '{}' is not utf8.",
                String::from_utf8_lossy(l)
            ))?;

            let (header, value) = str
                .split_once(":")
                .context(format!("Header line {} doesn't contain ':'", str))?;

            headers.insert(header.trim(), value.trim());
        }

        Ok(Self {
            major_ver,
            minor_ver,
            headers,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

impl TryFrom<&[u8]> for HttpMethod {
    type Error = anyhow::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(match value {
            b"GET" => Self::Get,
            b"HEAD" => Self::Head,
            b"POST" => Self::Post,
            b"PUT" => Self::Put,
            b"DELETE" => Self::Delete,
            b"CONNECT" => Self::Connect,
            b"OPTIONS" => Self::Options,
            b"TRACE" => Self::Trace,
            b"PATCH" => Self::Patch,
            other => {
                return Err(anyhow!(
                    "Invalid HTTP Method '{}'",
                    String::from_utf8_lossy(other)
                ))
            }
        })
    }
}
