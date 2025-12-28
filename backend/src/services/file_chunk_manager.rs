use std::collections::HashMap;

pub struct FileChunkHashes {
    map: HashMap<String, Vec<String>>,
}

impl FileChunkHashes {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    fn get_latest_chunk(&self, file_id: String) -> Option<&String> {
        match self.map.get(&file_id) {
            Some(data) => data.last(),
            None => None
        }
    }

    // complete this
    // fn set_latest_chunk(&mut self, latest_chunk: i32) {}

    fn get_hashes(&self, file_id: String) -> Option<&Vec<String>> {
        self.map.get(&file_id)
    }

    fn add_chunk_hash(&mut self, hash: String, file_id: String) {
        self.map.entry(file_id).and_modify(|v| v.push(hash.clone())).or_insert(vec![hash]);
    }
}
