use std::fs::File;
use crate::file::file_state::{FileChunk, FileState};

pub fn gen_mega_open(file_location: String, chunk_byte_size: u64) -> FileState {
    let file = File::open(&file_location).expect("Failed to open file");
    let file_metadata = file.metadata().expect("Failed to get file metadata");
    let file_size = file_metadata.len();
    
    let chunk_count = file_size / chunk_byte_size;
    let mut chunks: Vec<FileChunk> = Vec::new();
    create_chunks(chunk_byte_size, file_size, chunk_count, &mut chunks);
    
    let file_state = FileState {
        file_chunks: chunks,
        line_number: None,
        file_byte_size: file_size,
    };
    file_state
    
}

fn create_chunks(chunk_byte_size: u64, file_size: u64, chunk_count: u64, chunks: &mut Vec<FileChunk>) {
    for i in 0..chunk_count {
        let bytes_offset = i * chunk_byte_size;
        if i == chunk_count - 1 {
            // last chunk, may be smaller that chunk_byte_size
            let chunk = FileChunk {
                bytes_offset,
                size: file_size - (i * chunk_byte_size),
                line_number: None,
            };
            chunks.push(chunk);
            continue
        }
        let chunk = FileChunk {
            bytes_offset,
            size: chunk_byte_size,
            line_number: None,
        };
        chunks.push(chunk);
    }
}