#[derive(Debug)]
pub struct Elevator {
    pub y_old: f32,
    pub y_target: Option<i32>,
    pub y_velocity: f32,
    pub y: f32,
    pub door: f32,
}

impl Elevator {
    pub fn rectangle(&self) -> raylib::math::Rectangle {
        raylib::math::Rectangle::new(50.0, *&self.y as f32, 50.0, 80.0)
    }

    pub fn set_target(&mut self, y: i32) {
        self.y_old = self.y;
        self.y_target = Some(y);
    }
}

#[derive(Debug)]
pub struct StickFigure {
    pub x: f32,
    pub y: f32,
    pub walking: Option<i8>,
    pub in_elevator: bool,
}

impl StickFigure {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            walking: None,
            in_elevator: false,
        }
    }
}

#[derive(Debug)]
pub struct Floor {
    pub y: i32,
}

#[derive(Debug)]
pub struct State {
    pub elevator: Elevator,
    pub floors: Vec<Floor>,
    pub stick_figures: Vec<StickFigure>,
}

impl State {
    pub fn new() -> Self {
        State {
            elevator: Elevator {
                y_old: 0.0,
                y_target: None,
                y_velocity: 0.0,
                y: 50.0,
                door: 1.0,
            },
            floors: (1..8).map(|idx| Floor { y: (idx * 85) + 20 }).collect(),
            stick_figures: vec![],
        }
    }
}
