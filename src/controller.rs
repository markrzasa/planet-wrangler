use piston::ControllerAxisArgs;

#[derive(Clone, Copy)]
pub struct StickPosition {
    x: f64,
    y: f64,
}

impl StickPosition {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn get_x(self) -> f64 {
        self.x
    }

    pub fn get_y(self) -> f64 {
        self.y
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    left_stick: StickPosition,
    right_stick: StickPosition,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            left_stick: StickPosition::new(0.0, 0.0),
            right_stick: StickPosition::new(0.0, 0.0)
        }
    }

    pub fn get_left_stick(self) -> StickPosition {
        self.left_stick
    }

    pub fn get_right_stick(self) -> StickPosition {
        self.right_stick
    }

    pub fn update(&mut self, args: ControllerAxisArgs) {
        match args.axis {
            0 => self.left_stick.x = args.position,
            1 => self.left_stick.y = args.position,
            2 => self.right_stick.y = args.position,
            3 => self.right_stick.x = args.position,
            _ => {}
        }
    }
}
