use crate::services::{
    database_uploader::DatabaseUploader, file_details::FileDetails, ipfs_uploader::IPFSUploader,
};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, post,
    web::{self},
};
use socket2::{Domain, Socket, Type};
use std::net::SocketAddr;

mod database;
mod services;

#[post("/upload_file")]
async fn upload_file(
    file: web::Json<FileDetails>,
    database_uploader: web::Data<DatabaseUploader>,
) -> impl Responder {
    database_uploader.file_upload(file.0, 3).await.unwrap();
    HttpResponse::Ok().body("File received")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ipfs_uploader = IPFSUploader::new("https://rpc.filebase.io/api/v0/add");
    let database_uploader = DatabaseUploader::new("postgresql://neondb_owner:npg_wnGc5zvbaCH6@ep-empty-credit-a4il0l00-pooler.us-east-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require").await.unwrap();

    let address: SocketAddr = "127.0.0.1:4000".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.bind(&address.into())?;
    socket.listen(128)?;

    let listener = socket.into();
    println!("Starting the server...");

    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .app_data(web::PayloadConfig::new(10 * 1024 * 1024))
            .app_data(web::Data::new(ipfs_uploader.clone()))
            .app_data(web::Data::new(database_uploader.clone()))
    })
    .listen(listener)?
    .run()
    .await
}
