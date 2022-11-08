use crate::frame::Drawable;
use crate::frame::Frame;
/**
 * -------  ROW  -----
 * -------  ROW  -----
 *   |       |
 * Column  Column
 *   |       |
 */

#[derive(Clone, Debug)]
pub struct Section {
    pub x_start: usize,
    pub x_end: usize,
    pub y_start: usize,
    pub y_end: usize,
    pub messages: Vec<String>,
}

impl Section {
    pub fn new(x_start: usize, x_end: usize, y_start: usize, y_end: usize) -> Self {
        Self {
            x_start,
            x_end,
            y_start,
            y_end,
            messages: Vec::with_capacity(y_end - y_start - 1),
        }
    }

    fn get_capacity(&self) -> usize {
        self.y_end - self.y_start - 1
    }

    pub fn add_str(&mut self, str: &str) {
        self.add_message(str.to_string());
    }

    pub fn add_message(&mut self, message: String) {
        let capacity = self.get_capacity();
        if self.messages.len() == capacity {
            self.messages.remove(capacity - 1);
        }
        self.messages.insert(0, message);
    }

    pub fn draw_outline(&mut self, frame: &mut Frame) {
        for x in self.x_start..self.x_end {
            frame[x][self.y_start] = "-".to_string();
            frame[x][self.y_end] = "-".to_string();
        }

        for y in (self.y_start + 1)..self.y_end {
            frame[self.x_start][y] = "|".to_string();
            frame[self.x_end - 1][y] = "|".to_string();
        }
    }
}

impl Drawable for Section {
    fn draw(&self, frame: &mut Frame) {
        for (index, msg) in self.messages.iter().enumerate() {
            frame[self.x_start + 1][self.y_start + 1 + index] = msg.clone();
        }
    }
}
