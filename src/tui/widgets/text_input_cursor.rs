
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInputCursor {
    pub input_text: String,
    pub cursor_position: usize
}

impl TextInputCursor {
    pub fn new(input_text: String, cursor_position: usize) -> TextInputCursor {
        TextInputCursor { input_text, cursor_position }
    }

    pub fn new_empty() -> TextInputCursor {
        TextInputCursor { input_text: String::new(), cursor_position: 0 }
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input_text.insert(index, new_char);
        self.move_cursor_right()
    }
    
    pub fn delete_char_left(&mut self) {
        if self.cursor_position == 0 {
            return
        }

        let current_index = self.cursor_position;
        let before_current_index = current_index.saturating_sub(1);

        let before_char_to_delete = self.input_text.chars().take(before_current_index);
        let after_char_to_delete = self.input_text.chars().skip(current_index);

        self.input_text = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left()
    }

    pub fn delete_char_right(&mut self) {
        if self.cursor_position == self.input_text.chars().count() {
            return
        }

        let current_index = self.cursor_position;
        let after_current_index = current_index.saturating_add(1);

        let before_char_to_delete = self.input_text.chars().take(current_index);
        let after_char_to_delete = self.input_text.chars().skip(after_current_index);

        self.input_text = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_right()
    }

    pub fn move_cursor_left(&mut self) {
        let new_cursor_position = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(new_cursor_position);
    }

    pub fn move_cursor_right(&mut self) {
        let new_cursor_position = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(new_cursor_position);
    }

    pub fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn clamp_cursor(&self, new_position: usize) -> usize {
        new_position.clamp(0, self.input_text.len())
    }

    // letters can be multiple bytes.
    fn byte_index(&self) -> usize {
        self.input_text
            .char_indices()
            .map(|(index, _)| index)
            .nth(self.cursor_position)
            .unwrap_or_else(|| self.input_text.len())
    }
}