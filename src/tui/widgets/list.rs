use log::{info, warn};
use ratatui::widgets::ListState;

#[derive(Debug, Clone)]
pub struct SelectableList<T> {
    pub items: Vec<T>,
    pub state: ListState,
    pub horizontal_offset: usize,
    pub max_horizontal_offset: usize,
}

impl<T> Default for SelectableList<T> {
    // default implementation instantiates with an empty list and default table state.
    fn default() -> Self {
        Self::with_items(Vec::new())
    }
}
impl<T> SelectableList<T> {
    // Instantiate with items and a given table state.
    pub fn new(items: Vec<T>, mut state: ListState) -> Self {
        // If items.size == 0, it doesn't matter if we select 0 or None.
        state.select(Some(0));
        Self {
            items,
            state,
            horizontal_offset: 0,
            max_horizontal_offset: 100,
        }
    }

    // Instantiate with items but with a default/non-implemented table state.
    pub fn with_items(items: Vec<T>) -> Self {
        Self::new(items, ListState::default())
    }

    // Get the currently selected item, if any.
    // Not mutable, returns an Option reference to the item.
    pub fn get_selected_item(&self) -> Option<&T> {
        let selected_index = self.state.selected()?;
        self.items.get(selected_index)
    }

    pub fn set_horizontal_offset(&mut self, offset: usize) {
        if offset > self.max_horizontal_offset {
            warn!(
                "Trying to set horizontal offset to {} but max is {}.",
                offset, self.max_horizontal_offset
            );
            return;
        }
        info!("Setting horizontal offset to {}", offset);
        self.horizontal_offset = offset;
    }

    pub fn increase_horizontal_offset(&mut self, amount: usize) {
        let next_offset = self.horizontal_offset.saturating_add(amount);
        if next_offset > self.max_horizontal_offset {
            warn!(
                "Trying to increase horizontal offset to {} but max is {}.",
                next_offset, self.max_horizontal_offset
            );
            self.horizontal_offset = self.max_horizontal_offset;
        } else {
            info!("Increasing horizontal offset to {}", next_offset);
            self.horizontal_offset = next_offset;
        }
    }

    pub fn decrease_horizontal_offset(&mut self, amount: usize) {
        let next_offset = self.horizontal_offset.saturating_sub(amount);
        info!("Decreasing horizontal offset to {}", next_offset);
        self.horizontal_offset = next_offset;
    }

    pub fn force_select_item(&mut self, index: usize) {
        if self.items.len() <= index {
            warn!(
                "Trying to select item at index {} but list only has {} items.",
                index,
                self.items.len()
            );
        } else {
            info!("Forcing selection to index {}", index);
            self.state.select(Some(index))
        }
    }

    pub fn force_select_with_offset(&mut self, index: usize, offset: usize) {
        self.force_select_item(index);
        self.set_offset(offset);
    }

    pub fn force_select_first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn force_select_last(&mut self) {
        self.state.select(Some(self.items.len().saturating_sub(1)));
    }

    pub fn select_next(&mut self, amount: usize) {
        // fast path empty list
        if self.items.is_empty() {
            warn!("Selecting next item in the EMPTY list.");
            return self.state.select(Some(0));
        }

        let next_index = match self.state.selected() {
            Some(i) => {
                if i.saturating_add(amount) >= self.items.len() {
                    self.items.len().saturating_sub(1)
                } else {
                    i.saturating_add(amount)
                }
            }
            None => 0,
        };
        info!("select_next to index {}", next_index);
        self.state.select(Some(next_index))
    }

    pub fn select_prev(&mut self, amount: usize) {
        // fast path empty list
        if self.items.is_empty() {
            warn!("Selecting next item in the EMPTY list.");
            return self.state.select(Some(0));
        }
        let next_index = match self.state.selected() {
            Some(i) => {
                if i.saturating_sub(amount) >= self.items.len() {
                    0
                } else {
                    i.saturating_sub(amount)
                }
            }
            None => 0,
        };
        info!("select_prev to index {}", next_index);
        self.state.select(Some(next_index))
    }

    pub fn set_offset(&mut self, offset: usize) {
        if self.items.len() <= offset {
            warn!(
                "Trying to set offset to {} but list only has {} items.",
                offset,
                self.items.len()
            );
            return;
        }
        info!("Setting offset to {}", offset);
        *self.state.offset_mut() = offset;
    }

    pub fn increment_offset(&mut self, amount: usize) {
        if self.items.is_empty() {
            warn!("Selecting previous item in the EMPTY list.");
            return;
        }

        let current_offset = self.state.offset();
        let next_offset = if current_offset.saturating_add(amount) >= self.items.len() {
            // set to max offset
            self.items.len() - 1
        } else {
            current_offset.saturating_add(amount)
        };
        info!("increment_offset to {}", next_offset);
        *self.state.offset_mut() = next_offset;
    }

    pub fn decrement_offset(&mut self, amount: usize) {
        if self.items.is_empty() {
            warn!("Selecting previous item in the EMPTY list.");
            return;
        }

        let current_offset = self.state.offset();
        let next_offset = if current_offset.saturating_sub(amount) <= 0 {
            // set to 0
            0
        } else {
            current_offset.saturating_sub(amount)
        };
        info!("decrement_offset to {}", next_offset);
        *self.state.offset_mut() = next_offset;
    }
}
