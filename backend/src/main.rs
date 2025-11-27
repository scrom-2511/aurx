use crate::services::{ipfs_uploader, redis::RedisTaskQueue};
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, post,
    web::{self},
};
use serde::Deserialize;
use socket2::{Domain, Socket, Type};
use std::net::SocketAddr;

mod services;

#[derive(Deserialize)]
struct ChunkData {
    encrypted_chunk: String,
}

type RedisQueue = RedisTaskQueue;

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().json("hello sir")
}

#[post("/upload_chunk")]
async fn upload_chunk(
    data: web::Json<ChunkData>,
    redis_queue: web::Data<RedisQueue>,
) -> impl Responder {
    println!("I am called1");
    println!("{}", data.encrypted_chunk);
    let result = match redis_queue.push_task(&data.encrypted_chunk).await {
        Ok(_) => HttpResponse::Ok().json("Task queued successfully"),
        Err(e) => {
            eprintln!("Failed to queue task: {}", e);
            HttpResponse::InternalServerError().json("Failed to queue task")
        }
    };
    println!("I am called2");
    result
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let conn_for_push = client.get_multiplexed_async_connection().await.unwrap();

    let conn_for_pop = client.get_multiplexed_async_connection().await.unwrap();

    let redis_queue_for_handlers = RedisTaskQueue::new(conn_for_push.clone(), "task_queue");

    let redis_queue_for_worker = RedisTaskQueue::new(conn_for_pop, "task_queue");

    tokio::spawn(async move {
        loop {
            match redis_queue_for_worker.pop_task().await {
                Ok((_, val)) => match uploader.upload_chunk(val).await {
                    Ok(res) => {
                        println!("{}", res);
                    }
                    Err(e) => {
                        println!("Failed to upload chunk{}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to pop task: {}", e);
                }
            }
        }
    });

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
            .app_data(web::Data::new(redis_queue_for_handlers.clone()))
            .service(upload_chunk)
            .service(greet)
    })
    .listen(listener)?
    .run()
    .await
}
