pub enum InputButton {
    Left,
    Right,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
    L,
    R,
}

#[derive(Clone, Copy)]
pub struct InputButtons {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    a: bool,
    b: bool,
    select: bool,
    start: bool,
    l: bool,
    r: bool,
}

impl InputButtons {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            a: false,
            b: false,
            select: false,
            start: false,
            l: false,
            r: false,
        }
    }

    pub fn get_left(&self) -> bool {
        self.left
    }
    pub fn set_left(&mut self, value: bool) {
        self.left = value;
    }

    pub fn get_right(&self) -> bool {
        self.right
    }
    pub fn set_right(&mut self, value: bool) {
        self.right = value;
    }

    pub fn get_up(&self) -> bool {
        self.up
    }
    pub fn set_up(&mut self, value: bool) {
        self.up = value;
    }

    pub fn get_down(&self) -> bool {
        self.down
    }
    pub fn set_down(&mut self, value: bool) {
        self.down = value;
    }

    pub fn get_a(&self) -> bool {
        self.a
    }
    pub fn set_a(&mut self, value: bool) {
        self.a = value;
    }

    pub fn get_b(&self) -> bool {
        self.b
    }
    pub fn set_b(&mut self, value: bool) {
        self.b = value;
    }

    pub fn get_select(&self) -> bool {
        self.select
    }
    pub fn set_select(&mut self, value: bool) {
        self.select = value;
    }

    pub fn get_start(&self) -> bool {
        self.start
    }
    pub fn set_start(&mut self, value: bool) {
        self.start = value;
    }

    pub fn get_l(&self) -> bool {
        self.l
    }
    pub fn set_l(&mut self, value: bool) {
        self.l = value;
    }

    pub fn get_r(&self) -> bool {
        self.r
    }
    pub fn set_r(&mut self, value: bool) {
        self.r = value;
    }
}
