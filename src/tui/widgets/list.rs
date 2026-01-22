use ratatui::widgets::TableState;

#[derive(Debug)]
pub struct SelectableList<T> {
    pub items: Vec<T>,
    pub state: TableState,
}

impl<T> Default for SelectableList<T>  {

    // default implementation instantiates with an empty list and default table state.
    fn default() -> Self {
        Self::with_items(Vec::new())
    }
}
impl<T> SelectableList<T> {

    // Instantiate with items and a given table state.
    pub fn new(items: Vec<T>, mut state: TableState) -> Self {
        if items.is_empty() {
            state.select(None);
        } else {
            state.select(Some(0));
        }
        Self { items, state }
    }

    // Instantiate with items but with a default/non-implemented table state.
    pub fn with_items(items: Vec<T>) -> Self {
        Self::new(items, TableState::default())
    }

    // Get the currently selected item, if any.
    // Not mutable, returns an Option reference to the item.
    pub fn get_selected_item(&self) -> Option<&T> {
        let selected_index = self.state.selected()?;
        self.items.get(selected_index)
    }

    pub fn force_select_first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn force_select_last(&mut self) {
        self.state.select(Some(self.items.len().saturating_sub(1)));
    }

    pub fn select_next(&mut self, amount: usize) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }
        let last = self.items.len() - 1;
        let next_index = self.state.selected().unwrap_or(0).saturating_add(amount).min(last);
        self.state.select(Some(next_index));
    }

    pub fn select_prev(&mut self, amount: usize) {
        if self.items.is_empty() {
            self.state.select(None);
            return;
        }
        let next_index = self.state.selected().unwrap_or(0).saturating_sub(amount);
        self.state.select(Some(next_index));
    }
}
