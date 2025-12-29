
#[derive(Debug)]
pub struct State {
    pub ready: bool,
    pub should_quit: bool,
}

impl State {
    pub fn new() -> Self {
        Self { ready: false, should_quit: false }
    }
}