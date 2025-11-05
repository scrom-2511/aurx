use actix_web::{App, HttpServer, web};
use socket2::{Socket, Domain, Type};
use std::net::SocketAddr;

mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server...");

    let address: SocketAddr = "127.0.0.1:5000".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.bind(&address.into())?;
    socket.listen(128)?;

    let listener = socket.into();

    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .app_data(web::PayloadConfig::new(10*1024*1024))
    })
    .listen(listener)?
    .run()
    .await
}