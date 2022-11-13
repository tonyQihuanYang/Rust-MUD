use super::super::{MAP_X_END, MAP_X_START, MAP_Y_END, MAP_Y_START};
use super::ui_box::{UiBox, UiBoxMsg};
use crate::{
    position::Position,
    ui::frame::{Drawable, Frame, FrameMsg},
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Clone)]
pub struct MiniMapItem {
    pub id: u32,
    pub position: Position,
    // pub icon: FrameMsg,
    pub icon: String,
}

#[derive(Clone)]
pub struct MiniMap {
    pub items: Arc<RwLock<HashMap<u32, MiniMapItem>>>,
    pub ui: UiBox,
}

impl MiniMap {
    pub fn new() -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            ui: UiBox::new(MAP_X_START, MAP_X_END, MAP_Y_START, MAP_Y_END),
        }
    }

    pub fn show(&self, item: &MiniMapItem) {
        let mut items = self.items.write().unwrap();
        (*items).insert(item.id, item.clone());
    }

    pub fn remove(&self, id: &u32) {
        let mut items = self.items.write().unwrap();
        (*items).remove(id);
    }

    fn is_item_present(&self, item: &MiniMapItem) -> bool {
        let items = self.items.read().unwrap();
        (*items).contains_key(&item.id)
    }
}

impl Drawable for MiniMap {
    fn draw(&self, frame: &mut Frame) {
        self.ui.clear_frame(frame);
        self.ui.draw_outline(frame);

        let items = self.items.read().unwrap();

        let messages: Vec<UiBoxMsg> = (items)
            .iter()
            .map(|(_, value)| UiBoxMsg {
                value: FrameMsg::String(value.icon.clone()),
                position: Some(value.position.clone()),
            })
            .collect();
        self.ui.draw_frame(frame, &messages);
    }
}
