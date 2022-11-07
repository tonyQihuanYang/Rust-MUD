use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub type Frame = Vec<Vec<String>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(WINDOW_WIDTH);

    for _ in 0..WINDOW_WIDTH {
        let mut col = Vec::with_capacity(WINDOW_HEIGHT);
        for _ in 0..WINDOW_HEIGHT {
            col.push(" ".to_string());
        }
        cols.push(col)
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
