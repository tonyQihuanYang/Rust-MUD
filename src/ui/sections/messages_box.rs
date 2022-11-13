use super::ui_box::{UiBox, UiBoxMsg};
use crate::{
    position::Position,
    ui::frame::{Drawable, Frame, FrameMsg},
};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct MessagesBox {
    pub items: Arc<RwLock<Vec<UiBoxMsg>>>,
    pub ui: UiBox,
}

impl MessagesBox {
    pub fn new(x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Self {
        Self {
            items: Arc::new(RwLock::new(Vec::new())),
            ui: UiBox::new(x_start, x_end, y_start, y_end),
        }
    }

    pub fn show(&self, item: &UiBoxMsg) {
        let capacity = self.ui.get_y_capacity();
        let mut items = self.items.write().unwrap();
        if items.len() == capacity {
            items.remove(capacity - 1);
        }
        items.push(item.clone());
    }
}

impl Drawable for MessagesBox {
    fn draw(&self, frame: &mut Frame) {
        self.ui.draw_outline(frame);
        let items = self.items.read().unwrap();
        self.ui.draw_frame(frame, &items);
    }
}
