use piston::ControllerAxisArgs;

#[derive(Clone, Copy)]
pub struct StickPosition {
    degrees: f64,
    x: f64,
    y: f64,
    screen_x: f64,
    screen_y: f64,
    screen_width: f64,
    screen_height: f64
}

impl StickPosition {
    pub fn new(x: f64, y: f64, screen_width: f64, screen_height: f64) -> Self {
        Self {
            degrees: 0.0, x, y,
            screen_x: 0.0, screen_y: 0.0,
            screen_width, screen_height
        }
    }

    pub fn get_degrees(self) -> f64 {
        self.degrees
    }

    pub fn get_x(self) -> f64 {
        self.x
    }

    pub fn get_y(self) -> f64 {
        self.y
    }

    pub fn get_screen_x(self) -> f64 {
        self.screen_x
    }

    pub fn get_screen_y(self) -> f64 {
        self.screen_y
    }

    pub fn update(&mut self) {
        // analogue x and y axes move between -1 and 1
        self.screen_x = ((self.x - -1.0) / 2.0) * self.screen_width;
        self.screen_y = ((self.y - -1.0) / 2.0) * self.screen_height;
        self.degrees = self.y.atan2(self.x).to_degrees();
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    left_stick: StickPosition,
    right_stick: StickPosition
}

impl Controller {
    pub fn new(screen_width: f64, screen_height: f64) -> Self {
        Self {
            left_stick: StickPosition::new(0.0, 0.0, screen_width, screen_height),
            right_stick: StickPosition::new(0.0, 0.0, screen_width, screen_height)
        }
    }

    pub fn get_left_stick(self) -> StickPosition {
        self.left_stick
    }

    pub fn get_right_stick(self) -> StickPosition {
        self.right_stick
    }

    pub fn update(&mut self, args: ControllerAxisArgs) {
        let position = (args.position * 10.0).trunc() / 10.0;
        match args.axis {
            0 => self.left_stick.x = position,
            1 => self.left_stick.y = position,
            2 => self.right_stick.x = position,
            3 => self.right_stick.y = position,
            _ => {}
        }

        self.left_stick.update();
        self.right_stick.update();
    }
}
