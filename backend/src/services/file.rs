use actix_web::cookie::time::OffsetDateTime;

use crate::types::file_type::FileType;

struct File<T>{
    name: String,
    file_size: u32,
    file_type: FileType,
    file_extension: T,
    uploaded_at: OffsetDateTime
}