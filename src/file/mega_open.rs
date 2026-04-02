use crate::file::file_state::{FileChunk, FileState};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

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

fn create_chunks(
    chunk_byte_size: u64,
    file_size: u64,
    chunk_count: u64,
    chunks: &mut Vec<FileChunk>,
) {
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
            continue;
        }
        let chunk = FileChunk {
            bytes_offset,
            size: chunk_byte_size,
            line_number: None,
        };
        chunks.push(chunk);
    }
}

// seek the chunk in the file and count the number of lines present in the chunk. count only line separators, not the actual lines.
fn calculate_n_set_chunk_lines(file_location: String, file_chunk: &mut FileChunk) {
    let file = File::open(&file_location).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(file);
    reader
        .seek(SeekFrom::Start(file_chunk.bytes_offset))
        .expect("Failed to seek in file");
    let mut line_count = 0;
    let mut buffer = vec![0; file_chunk.size as usize];
    reader
        .read_exact(&mut buffer)
        .expect("Failed to read chunk");
    for byte in buffer {
        if byte == b'\n' {
            line_count += 1;
        }
    }
    file_chunk.line_number = Some(line_count as u64);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::thread;
    use std::time::Instant;

    const BIG_FILE: &str = "./test-logs/numbers-hundred-milion.log";
    static CHUNK_BYTE_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

    // 124µs
    #[test]
    fn test_gen_mega_open_chunking_e2e() {
        let chunk_byte_size = CHUNK_BYTE_SIZE;
        let file_size = File::open(BIG_FILE)
            .expect("failed to open big test file")
            .metadata()
            .expect("failed to read big test file metadata")
            .len();
        let expected_chunk_count = file_size / chunk_byte_size;

        let start = Instant::now();
        let result = gen_mega_open(BIG_FILE.to_string(), chunk_byte_size);
        println!(
            "test_gen_mega_open_chunking_e2e execution took: {:?}",
            start.elapsed()
        );

        assert_eq!(result.file_byte_size, file_size);
        assert_eq!(result.line_number, None);
        assert_eq!(result.file_chunks.len() as u64, expected_chunk_count);
        if expected_chunk_count > 0 {
            let first_chunk = &result.file_chunks[0];
            let last_chunk = result
                .file_chunks
                .last()
                .expect("expected at least one chunk");
            assert_eq!(first_chunk.bytes_offset, 0);
            assert_eq!(first_chunk.size, chunk_byte_size);
            assert_eq!(
                last_chunk.bytes_offset,
                (expected_chunk_count - 1) * chunk_byte_size
            );
            assert_eq!(
                last_chunk.size,
                file_size - ((expected_chunk_count - 1) * chunk_byte_size)
            );
        }
    }

    // 181.1115ms
    #[test]
    fn test_calculate_n_set_chunk_lines_e2e() {
        let chunk_byte_size = CHUNK_BYTE_SIZE;
        let mut chunk = FileChunk {
            bytes_offset: 0,
            size: chunk_byte_size,
            line_number: None,
        };

        let start = Instant::now();
        calculate_n_set_chunk_lines(BIG_FILE.to_string(), &mut chunk);
        println!(
            "test_calculate_n_set_chunk_lines_e2e execution took: {:?}",
            start.elapsed()
        );

        assert!(chunk.line_number.is_some());
        assert!(chunk.line_number.expect("line_number should be set") > 0);
    }

    // 74µs
    #[test]
    fn test_gen_mega_open_large_chunk_empty_result_behavior() {
        let file_size = File::open(BIG_FILE)
            .expect("failed to open big test file")
            .metadata()
            .expect("failed to read big test file metadata")
            .len();
        let start = Instant::now();
        let result = gen_mega_open(BIG_FILE.to_string(), file_size.saturating_mul(2));
        println!(
            "test_gen_mega_open_large_chunk_empty_result_behavior execution took: {:?}",
            start.elapsed()
        );

        assert_eq!(result.file_byte_size, file_size);
        assert!(result.file_chunks.is_empty());
    }

    // 3.1434687s
    #[test]
    fn test_gen_then_calculate_lines_for_all_chunks_e2e() {
        let chunk_byte_size = CHUNK_BYTE_SIZE;
        let start = Instant::now();
        let mut file_state = gen_mega_open(BIG_FILE.to_string(), chunk_byte_size);
        assert!(
            !file_state.file_chunks.is_empty(),
            "expected at least one chunk for BIG_FILE with 512MB chunk size"
        );

        let workers = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
            .max(2);

        for batch in file_state.file_chunks.chunks_mut(workers) {
            thread::scope(|scope| {
                for chunk in batch {
                    scope.spawn(move || {
                        calculate_n_set_chunk_lines(BIG_FILE.to_string(), chunk);
                    });
                }
            });
        }

        println!(
            "test_gen_then_calculate_lines_for_all_chunks_e2e execution took: {:?}",
            start.elapsed()
        );
        println!("the file state {:?}", file_state);

        assert!(
            file_state
                .file_chunks
                .iter()
                .all(|chunk| chunk.line_number.is_some())
        );
        assert!(
            file_state
                .file_chunks
                .iter()
                .map(|chunk| chunk.line_number.expect("line_number should be set"))
                .sum::<u64>()
                > 0
        );
    }
}
