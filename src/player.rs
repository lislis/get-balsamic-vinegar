pub struct Player {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    speed: f64,
    is_jumping: bool,
    max_height: f64,
    ground_pos: f64,
    pub basket: Vec<String>
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 60.0,
            y: 300.0,
            w: 120.0,
            h: 80.0,
            speed: 1.0,
            is_jumping: false,
            max_height: 140.0,
            ground_pos: 300.0, // same as y at init
            basket: vec![]
        }
    }
    pub fn update(&mut self, dt: f64) {
        if self.is_jumping {
            self.y -= self.speed;
            if self.y < self.max_height {
                self.is_jumping = false;
            }
        }
        if !self.is_jumping && self.y < self.ground_pos {
            self.y += self.speed;
        }

    }
    pub fn jump(&mut self) {
        // println!("jumping");
        self.is_jumping = true;
    }
    pub fn buying(&mut self, item: String) {
        println!("{}", item);
        self.basket.push(item);
    }
}
