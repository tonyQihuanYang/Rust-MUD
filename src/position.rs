/**
 *  x, y (0, 0)
 *  |-------- (10, 0)
 *  |
 *  |
 *  |
 *  (0,10)
 */

#[derive(Clone, Debug)]
pub struct Bound {
    pub x_s: usize,
    pub x_e: usize,
    pub y_s: usize,
    pub y_e: usize,
}

impl Bound {
    pub fn new(x_s: usize, x_e: usize, y_s: usize, y_e: usize) -> Self {
        Self { x_s, x_e, y_s, y_e }
    }

    pub fn is_in_bound(&self, position: &Position) -> bool {
        self.is_x_in_bound(&position.x) && self.is_y_in_bound(&position.y)
    }

    pub fn is_x_in_bound(&self, x: &usize) -> bool {
        (*x > self.x_s) && (*x < self.x_e)
    }

    pub fn is_y_in_bound(&self, y: &usize) -> bool {
        (*y > self.y_s) && (*y < self.y_e)
    }
}

#[derive(Clone, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub bound: Option<Bound>,
}

impl Position {
    pub fn new(x: usize, y: usize, bound: Option<Bound>) -> Self {
        Self { x, y, bound }
    }

    pub fn move_left(&mut self) {
        match self.bound.clone() {
            Some(bound) => {
                if bound.clone().is_x_in_bound(&(self.x - 1)) {
                    self.x -= 1;
                }
            }
            _ => self.x -= 1,
        };
    }

    pub fn move_right(&mut self) {
        match self.bound.clone() {
            Some(bound) => {
                if bound.is_x_in_bound(&(self.x + 1)) {
                    self.x += 1;
                }
            }
            _ => self.x += 1,
        };
    }

    pub fn move_up(&mut self) {
        match self.bound.clone() {
            Some(bound) => {
                if bound.is_y_in_bound(&(self.y - 1)) {
                    self.y -= 1;
                }
            }
            _ => self.y -= 1,
        };
    }

    pub fn move_down(&mut self) {
        match self.bound.clone() {
            Some(bound) => {
                if bound.is_y_in_bound(&(self.y + 1)) {
                    self.y += 1;
                }
            }
            _ => self.y += 1,
        };
    }
}
