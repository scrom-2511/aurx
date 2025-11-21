use crate::services::{ipfs_uploader, redis::RedisTaskQueue};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, post,
    web::{self},
};
use serde::Deserialize;
use socket2::{Domain, Socket, Type};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

mod services;

#[derive(Deserialize)]
struct ChunkData {
    encrypted_chunk: String,
}

type RedisQueue = Arc<Mutex<RedisTaskQueue>>;

#[post("/upload_chunk")]
async fn upload_chunk(
    data: web::Json<ChunkData>,
    redis_queue: web::Data<RedisQueue>,
) -> impl Responder {
    redis_queue
        .lock()
        .await
        .push_task(&data.encrypted_chunk)
        .await
        .ok();
    HttpResponse::Ok().json("great")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server...");

    let uploader = ipfs_uploader::IPFSUploader::new("https://rpc.filebase.io/api/v0/add");

    let client = match redis::Client::open(
        "redis://default:uL6N1puUYwFOKOWSHiEfWFrgUSNzP6TT@redis-12732.c232.us-east-1-2.ec2.cloud.redislabs.com:12732",
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Redis connection failed: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Redis init failed",
            ));
        }
    };

    let connection = match client.get_multiplexed_async_connection().await {
        Ok(connection) => connection,
        Err(e) => {
            eprintln!("Redis client fetching failed: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Redis client failed.",
            ));
        }
    };

    let redis_queue = Arc::new(Mutex::new(RedisTaskQueue::new(
        connection,
        "task_queue",
        uploader,
    )));
    println!("Hi there2");
    tokio::spawn({
        let redis_queue = redis_queue.clone();
        async move {
            redis_queue.lock().await.pop_task().await.ok();
        }
    });

    let address: SocketAddr = "127.0.0.1:5000".parse().unwrap();

    let socket = Socket::new(Domain::IPV4, Type::STREAM, None)?;
    socket.set_reuse_address(true)?;
    socket.bind(&address.into())?;
    socket.listen(128)?;
    println!("Hi there2");

    let listener = socket.into();
    println!("Hi there3");

    HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allow_any_header(),
            )
            .app_data(web::PayloadConfig::new(10 * 1024 * 1024))
            .app_data(web::Data::new(redis_queue.clone()))
            .app_data(web::Data::new("hit this is me"))
            .service(upload_chunk)
    })
    .listen(listener)?
    .run()
    .await
}
