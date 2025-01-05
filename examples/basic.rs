use super::{
    res::{HttpRes, StatusCode},
    HttpServer,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = HttpServer::new("0.0.0.0:80")?;

    server.listen(|_| HttpRes::new(StatusCode::Ok, Some("Hello")));

    Ok(())
}
