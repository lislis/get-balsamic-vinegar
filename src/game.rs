extern crate rand;

use player::Player;
use buyable::Buyable;
use rand::Rng;

pub struct Game {
    pub state: &'static str,
    pub energy: f64,
    pub relationship: f64,
    pub player: Player,
    pub buyables: Vec<Buyable>,
    items: Vec<String>,
    list: Vec<String>,
    t: f64,
    t_spawn: f64,
    t_shop: f64,
    pub bought_correctly: Vec<String>,
    pub bought_too_much: usize,
    pub not_there: usize
}

impl Game {
    pub fn new(param_state: &'static str) -> Game {
        Game {
            state: param_state,
            energy: 10.0,
            relationship: 10.0,
            player: Player::new(),
            buyables: vec![],
            items: vec![
                "field salad".to_string(),
                "tomatoes".to_string(),
                "onions".to_string(),
                "olive oil".to_string(),
                "balsamic vinegar".to_string(),
                "salt".to_string(),
                "black pepper".to_string(),
                "mustard".to_string(),
                "chocolate".to_string(),
                "ketchup".to_string(),
                "bread".to_string(),
                "cheese".to_string(),
                "beer".to_string(),
                "apples".to_string(),
            ],
            list: vec![
                "field salad".to_string(),
                "tomatoes".to_string(),
                "onions".to_string(),
                "olive oil".to_string(),
                "balsamic vinegar".to_string(),
                "salt".to_string(),
                "black pepper".to_string(),
                "mustard".to_string()
            ],
            t: 0.0,
            t_spawn: 1.5,
            t_shop: 20.0,
            bought_correctly: vec![],
            bought_too_much: 0,
            not_there: 0
        }
    }
    pub fn set_state(&mut self, state: &'static str) {
        self.state = state;
    }
    pub fn update(&mut self, dt: f64) {

        self.t += dt;
        self.t_spawn += dt;

        if self.t_spawn > 2.2 { // should probably be a variable
            self.t_spawn = 0.0;
            self.spawn_buyable();
        }

        if self.t > self.t_shop {
            self.t = 0.0;
            self.compare_groceries();
            self.set_state(&"compare");
        }

        self.player.update(dt);

        for b in self.buyables.iter_mut() {
            b.update(dt);
        }
    }
    pub fn collision_detection(&mut self) {
        for b in self.buyables.iter_mut() {
            if b.is_visible {
                if self.player.x < b.x + b.w &&
                    self.player.x + self.player.w > b.x &&
                    self.player.y < b.y + b.h &&
                    self.player.y + self.player.h > b.y {
                        //  println!("COLLIDE");
                        b.into_basket();
                        self.player.buying(b.id.to_string());
                    }
            }
        }
    }

    pub fn spawn_buyable(&mut self) {
        let item = rand::thread_rng().choose(&self.items).unwrap();

        self.buyables.push(Buyable::new(item.to_string()));
    }

    pub fn compare_groceries(&mut self) {
        for bought in self.player.basket.iter_mut() {
            for needed in self.list.iter_mut() {
                if bought == needed {
                    self.bought_correctly.push(bought.to_string());
                }
            }
        }

        self.bought_too_much = self.player.basket.len() - self.bought_correctly.len();
        self.not_there = self.list.len() - self.bought_correctly.len();

        self.update_game_status();
    }

    pub fn update_game_status(&mut self) {
        self.energy -= 1.0;
        self.relationship += self.bought_correctly.len() as f64;
        self.relationship -= self.not_there as f64;
        println!("{}", self.energy);
        println!("{}", self.relationship);
    }

    pub fn reset_for_shop(&mut self) {
        self.buyables = vec![];
        self.player.basket = vec![];
    }
}
