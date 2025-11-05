use actix_web::cookie::time::OffsetDateTime;
use std::fs::FileType;

struct FileDetails {
    name: String,
    file_size: u32,
    file_type: FileType,
    file_extension: String,
    latest_chunk: i32,
    uploaded_at: OffsetDateTime,
}

struct FileDetailsWithHash {
    file_details: FileDetails,
    hashes: Vec<String>,
}

impl FileDetailsWithHash {
    pub fn new(file_details: FileDetails, hash: String) -> Self {
        Self {
            file_details,
            hashes: vec![hash],
        }
    }

    fn get_latest_chunk(&self) -> i32 {
        self.file_details.latest_chunk
    }

    fn set_latest_chunk(&mut self, latest_chunk: i32) {
        self.file_details.latest_chunk = latest_chunk;
    }

    fn get_hashes(&self) -> &Vec<String> {
        &self.hashes
    }

    fn add_hash(&mut self, hash: String) {
        self.hashes.push(hash);
    }
}