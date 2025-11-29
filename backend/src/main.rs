use crate::services::{
    ipfs_uploader::{self, IPFSUploader}, redis_connection::RedisConnection, redis_task::RedisTaskQueue,
};
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, post, rt,
    web::{self},
};
use actix_ws::Message;
use futures_util::StreamExt;
use redis::RedisError;
use serde::Deserialize;
use socket2::{Domain, Socket, Type};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    net::SocketAddr,
};

mod services;

#[macro_export]
macro_rules! handle_err {
    ($expr:expr) => {{
        if let Err(e) = $expr {
            eprintln!("{}", e);
        }
    }};
}

#[derive(Deserialize)]
struct ChunkData {
    encrypted_chunk: String,
}

type RedisQueue = RedisTaskQueue;

#[derive(Deserialize)]
struct WebsocketVal {
    hash: String,
    latest_chunk: i32,
    total_chunks: i32,
    file_id: String,
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

async fn redis_initialize() -> Result<RedisTaskQueue, RedisError> {
    let redis_conn = RedisConnection::new("redis://default:uL6N1puUYwFOKOWSHiEfWFrgUSNzP6TT@redis-12732.c232.us-east-1-2.ec2.cloud.redislabs.com:12732".to_string())?;

    let connections = redis_conn
        .get_handler_and_worker_connection()
        .await
        .unwrap();

    let redis_queue = RedisTaskQueue::new(connections, "task_queue");
    Ok(redis_queue)
}

async fn ws_handler(req: HttpRequest, body: web::Payload, ipfs_uploader: web::Data<IPFSUploader>) -> Result<HttpResponse, Error> {
    let (response, mut session, mut stream) = actix_ws::handle(&req, body)?;

    rt::spawn(async move {
        while let Some(Ok(msg)) = stream.next().await {
            match msg {
                Message::Text(text) => match serde_json::from_str::<WebsocketVal>(&text) {
                    Ok(val) => {
                        match OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(format!("./users/files/{}.txt", val.file_id))
                        {
                            Ok(mut file) => {
                                handle_err!(write!(file, "{},", val.hash));
                                if val.latest_chunk == val.total_chunks {
                                    handle_err!(ipfs_uploader.upload(&val.file_id, &format!("./users/files/{}.txt", val.file_id)).await);
                                    handle_err!(fs::remove_file(format!("./users/files/{}.txt", val.file_id)));
                                }
                            }
                            Err(e) => eprint!("{}", e),
                        }
                    }
                    Err(err) => eprintln!("Invalid JSON: {}, raw text: {}", err, text),
                },
                Message::Ping(bytes) => {
                    let _ = session.pong(&bytes).await;
                }
                Message::Close(reason) => {
                    println!("Client disconnected: {:?}", reason);
                    let _ = session.close(reason).await;
                    break;
                }
                _ => {}
            }
        }
    });

    Ok(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let uploader = IPFSUploader::new("https://rpc.filebase.io/api/v0/add");

    let redis_queue = redis_initialize()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let redis_queue_for_worker = redis_queue.clone();

    // tokio::spawn(async move {
    //     loop {
    //         match redis_queue_for_worker.pop_task().await {
    //             Ok((_, val)) => match uploader.upload_chunk(val).await {
    //                 Ok(res) => {
    //                     println!("{}", res);
    //                 }
    //                 Err(e) => {
    //                     println!("Failed to upload chunk{}", e);
    //                 }
    //             },
    //             Err(e) => {
    //                 eprintln!("Failed to pop task: {}", e);
    //             }
    //         }
    //     }
    // });

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
            .app_data(web::Data::new(redis_queue.clone()))
            .service(upload_chunk)
            .route("/ws", web::get().to(ws_handler))
    })
    .listen(listener)?
    .run()
    .await
}
