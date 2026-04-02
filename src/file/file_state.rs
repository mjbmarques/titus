#[derive(Debug, Clone)]
pub struct FileState {
    pub file_chunks: Vec<FileChunk>,
    pub line_number: Option<u64>,
    pub file_byte_size: u64,
}

#[derive(Debug, Clone)]
pub struct FileChunk {
    pub bytes_offset: u64,
    pub size: u64,
    pub line_number: Option<u64>,
    // ----------- //
    // Prob temporary, keep only if better performance than calculating on the fly.
    // buff_reader: BufReader<File>,
    // ----------- //
}
