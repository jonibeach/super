use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Context;

use crate::util::{HttpInner, HttpMethod};

#[derive(Debug)]
pub struct HttpReq<'a> {
    pub inner: HttpInner<'a>,
    pub method: HttpMethod,
    pub path: &'a str,
    pub body: Option<&'a [u8]>,
    _raw: &'a [u8]
}

impl<'a> HttpReq<'a> {
    pub fn from_reader(raw: &'a mut Vec<u8>, r: &'a mut impl Read) -> anyhow::Result<Self> {
        let mut r = BufReader::new(r);

        let mut buf = Vec::new();
        loop {
            // Minus 2 from len as all lines include \r\n at the end
            match r.read_until(b'\n', &mut buf)? - 2 {
                0 => {
                    break;
                }
                _ => raw.write_all(&buf)?,
            }
        }

        let mut lines: Vec<&[u8]> = raw.split(|b| *b == b'\n').collect();

        let request_line = lines.get(0).with_context(||"Request line not found")?;

        let request_line_parts: Vec<&[u8]> = request_line.split(|b| b == &b' ').collect();
        let [method, path, ver]: [&[u8]; 3] = request_line_parts
            .chunks_exact(3)
            .next()
            .with_context(||"Request line parts don't exist")?
            .try_into()?;

        // This will be parsed in inner
        lines[0] = ver;

        let method = HttpMethod::try_from(method)?;
        let path = core::str::from_utf8(path)?;
        let inner = HttpInner::from_byte_lines(&lines)?;

        let mut body = None;
        if let Some(last) = lines.last() {
            if !last.is_empty() {
                body = Some(*last);
            }
        }

        Ok(Self {
            method,
            path,
            inner,
            body,
            _raw: &raw[..],
        })
    }
}
