use tui::{
    backend::Backend,
    layout::Rect,
    Frame
};

pub trait Tickable {
    fn tick(&mut self);
}

pub trait DebugUI {
    fn draw<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect);
}
