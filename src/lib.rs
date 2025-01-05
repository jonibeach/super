use std::{
    io::Write,
    net::{TcpListener, ToSocketAddrs},
    sync::Arc,
    thread,
};

use req::HttpReq;
use res::{HttpRes, IntoHttpRes, StatusCode};

pub mod req;
pub mod res;
pub mod util;

pub struct HttpServer {
    l: TcpListener,
}

impl HttpServer {
    pub fn new(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
        Ok(Self {
            l: TcpListener::bind(addr)?,
        })
    }
    pub fn listen<'a, F, R>(&self, on_req: F)
    where
        F: Send + Sync + 'static + Fn(&HttpReq) -> R,
        R: IntoHttpRes<'a>,
    {
        let on_req = Arc::new(on_req);
        for s in self.l.incoming() {
            let on_req = Arc::clone(&on_req);
            thread::spawn(move || {
                let mut s = match s {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Error in stream: {e}");
                        return;
                    }
                };

                let mut raw = Vec::new();
                let req = match HttpReq::from_reader(&mut raw, &mut s) {
                    Ok(r) => r,
                    Err(e) => {
                        let res = HttpRes::new(
                            StatusCode::BadRequest,
                            Some(format!("Malformed request: {e}")),
                        );
                        let _ = s.write_all(&res.to_bytes());
                        return;
                    }
                };

                let res = on_req(&req).into_res();
                let _ = s.write_all(&res.to_bytes());
            });
        }
    }
}
