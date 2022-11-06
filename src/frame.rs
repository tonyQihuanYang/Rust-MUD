use crate::{NUM_COLS, NUM_ROWS};

// pub type Frame = Vec<Vec<&'static str>>;
pub type Frame = Vec<Vec<String>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);

    for _ in 0..NUM_COLS {
        let mut col = Vec::with_capacity(NUM_ROWS);
        for _ in 0..NUM_ROWS {
            col.push(" ".to_string());
        }
        cols.push(col)
    }
    cols

    // let mut cols = Vec::with_capacity(WINDOW_WIDTH);

    // for _ in 0..WINDOW_WIDTH {
    //     let mut col = Vec::with_capacity(WINDOW_HEIGHT);
    //     for _ in 0..WINDOW_HEIGHT {
    //         col.push(" ");
    //     }
    //     cols.push(col)
    // }
    // cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
