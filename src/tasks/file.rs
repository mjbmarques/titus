use crate::file;
use crate::tui::file_io;

pub fn open_file(file_location: String) -> Vec<String> {
    let mut lines_vec: Vec<String> = Vec::new();
    if let Ok(lines) = file_io::read_lines(file_location) {
        for line in lines.map_while(Result::ok) {
            lines_vec.push(line);
        }
    };
    lines_vec
}

pub fn find_in_file(file_location: String, query: String) -> Vec<(String, bool)> {
    if let Ok(result) = file::find::find_in_file_with_rg(file_location, query, 1000, 1000) {
        return result;
    }
    vec![]
}
