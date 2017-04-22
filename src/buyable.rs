pub struct Buyable {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    speed: f64,
    pub id: String,
    pub is_visible: bool
}

impl Buyable {
    pub fn new(param_id: String) -> Buyable {
        Buyable {
            x: 500.0, // edge of the screen
            y: 300.0, // same as player
            w: 50.0,
            h: 50.0,
            speed: 3.0,
            id: param_id,
            is_visible: true
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.x > 0.0 - 50.0 {
            self.x -= 1.0 + self.speed * dt;
        } else {
            self.is_visible = false;
        }
        // deactivate
    }

    pub fn into_basket(&mut self) {
        self.is_visible = false;
    }
}
