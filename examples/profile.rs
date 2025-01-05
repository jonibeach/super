use super_http::{req::HttpReq, res::{HttpRes, StatusCode}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = include_bytes!("../random_http_requests.txt");

    for mut r in bytes.split(|b| *b == 0) {
        let mut vec = Vec::new();
        let req = HttpReq::from_reader(&mut vec, &mut r)?;
        let res = HttpRes::new(StatusCode::Ok, req.body);
        let _ = res.to_bytes();
    }

    Ok(())
}
