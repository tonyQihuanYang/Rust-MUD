use crate::{MONSTERS_LIST_X, MONSTERS_LIST_Y, WINDOW_HEIGHT, WINDOW_WIDTH};

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

pub fn clear_monster_list(cur_frame: &mut Frame) {
    for y in MONSTERS_LIST_Y..WINDOW_HEIGHT {
        cur_frame[MONSTERS_LIST_X][y] = " ".to_string().repeat(WINDOW_WIDTH - MONSTERS_LIST_X);
    }
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
