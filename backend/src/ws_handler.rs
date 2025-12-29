use std::{fs::File, io::Write, sync::Arc};

use actix_web::{Error, HttpRequest, HttpResponse, rt, web};
use actix_ws::Message;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{services::{file_chunk_manager::FileChunkHashes, ipfs_uploader::IPFSUploader, solana_handler::SolanaHandler}};

#[derive(Serialize, Deserialize)]
struct WebsocketVal {
    hash: String,
    latest_chunk: u32,
    total_chunks: u32,
    file_id: String,
    user_id: u128
}

fn ws_handler(
    req: HttpRequest,
    body: web::Payload,
    file_chunks_manager: web::Data<Arc<Mutex<FileChunkHashes>>>,
    ipfs_uploader: web::Data<IPFSUploader>,
) -> Result<HttpResponse, Error> {
    let (res, _session, mut stream) = actix_ws::handle(&req, body)?;

    let file_chunks_manager_cloned = file_chunks_manager.clone();

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(data) = serde_json::from_str::<WebsocketVal>(&text) {
                        file_chunks_manager_cloned
                            .lock()
                            .await
                            .add_chunk_hash(&data.hash, &data.file_id);

                        if data.latest_chunk == data.total_chunks {
                            let chunks_string = file_chunks_manager_cloned
                                .lock()
                                .await
                                .get_hashes(&data.file_id)
                                .unwrap()
                                .join(",");
                            let file_path = format!("../../users/files/{}", data.file_id);
                            let mut file = File::create(file_path.clone()).unwrap();
                            file.write_all(chunks_string.as_bytes()).unwrap();
                            let file_hash = ipfs_uploader
                                .upload(&data.file_id, &file_path)
                                .await
                                .unwrap();
                            SolanaHandler::add_new_file_hash(&data.file_id, &file_hash, data.user_id);
                        }
                    }
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => {}
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            }
        }
    });

    Ok(res)
}
