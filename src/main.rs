extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;
extern crate rand;

mod player;
mod buyable;

use std::rc::Rc;
use std::path::PathBuf;
use rand::Rng;
use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait
};
use player::Player;
use buyable::Buyable;

struct Game {
    state: &'static str,
    energy: f64,
    relationship: f64,
    player: Player,
    buyables: Vec<Buyable>,
    items: Vec<String>,
    list: Vec<String>,
    t: f64,
    t_spawn: f64,
    t_shop: f64,
    bought_correctly: Vec<String>,
    bought_too_much: usize,
    not_there: usize
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

fn get_asset_path(filename: &'static str) -> PathBuf {
    find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap()
        .join(filename)
}

fn main() {

    let (width, height) = (600, 500);
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow =
        WindowSettings::new("Get balsamic vinegar", (width, height))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let bubble_id;
    let bubble_2;
    let bubble_3;
    let bubble_4;
    let mut talk_scene = Scene::new();

    let tex = Rc::new(Texture::from_path(
        &mut window.factory,
        get_asset_path("bubble.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());
    let tex2 = Rc::new(Texture::from_path(
        &mut window.factory,
        get_asset_path("bubble.png"),
        Flip::Horizontal,
        &TextureSettings::new()).unwrap());

    let mut bubble = Sprite::from_texture(tex.clone());
    let mut bubble2 = Sprite::from_texture(tex2.clone());
    let mut bubble3 = Sprite::from_texture(tex.clone());
    let mut bubble4 = Sprite::from_texture(tex2.clone());

    bubble.set_position(200.0, 100.0);
    bubble2.set_position(400.0, 200.0);
    bubble3.set_position(200.0, 300.0);
    bubble4.set_position(400.0, 400.0);

    bubble_id = talk_scene.add_child(bubble);
    bubble_2 = talk_scene.add_child(bubble2);
    bubble_3 = talk_scene.add_child(bubble3);
    bubble_4 = talk_scene.add_child(bubble4);

    let talk_seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.4, 0.4))))
    ]);
    let talk_seq2 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(2.5),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.4, 0.4))))
    ]);
    let talk_seq3 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(5.0),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.4, 0.4))))
    ]);
    let talk_seq4 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(7.5),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.4, 0.4))))
    ]);
    talk_scene.run(bubble_id, &talk_seq);
    talk_scene.run(bubble_2, &talk_seq2);
    talk_scene.run(bubble_3, &talk_seq3);
    talk_scene.run(bubble_4, &talk_seq4);

    let color = [0.0, 1.0, 1.0, 1.0];
    let mut glyphs = Glyphs::new(
        get_asset_path("shpinscher-regular.ttf"),
        window.factory.clone()).unwrap();

    let mut game = Game::new(&"start");

    let player = Texture::from_path(
        &mut window.factory,
        &get_asset_path(&"rust.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    while let Some(e) = window.next() {

        if game.state == "talk" {
            talk_scene.event(&e);
        }

        match e {
            Input::Release(Button::Keyboard(key)) => {

                if key == Key::Space {
                    match game.state {
                        "start" => {
                            game.set_state(&"talk");
                        }
                        "talk" => {
                            game.set_state(&"status");
                        }
                        "status" => {
                            if game.energy >= game.relationship {
                                game.reset_for_shop();
                                game.set_state(&"shop");
                            } else {
                                game.set_state(&"game_over");
                            }
                        }
                        "shop" => {
                            game.player.jump();
                        }
                        "compare" => {
                            if game.not_there == 0 {
                                game.set_state(&"win");
                            } else {
                                game.set_state(&"resume");
                            }
                        }
                        "resume" => {
                            game.set_state(&"status");
                        }
                        _ => {}
                    }
                }
            }

            Input::Update(args) => {
                match game.state {
                    "shop" => {
                        game.update(args.dt);
                        game.collision_detection();
                    }
                    _ => {}
                }
            }

            Input::Render(_) => {

                match game.state {
                    "start" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 1.0, 1.0, 1.0], g);

                            text::Text::new_color(color, 30).draw(
                                "Get balsamic vinegar",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(color, 30).draw(
                                "Press <space> to start",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);
                        });
                    }
                    "talk" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 1.0, 1.0, 1.0], g);

                            talk_scene.draw(c.transform, g);
                            text::Text::new_color(color, 30).draw(
                                "Press <space> to go",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    "shop" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 0.0, 0.0, 1.0], g);
                            image(&player, c.transform.trans(game.player.x, game.player.y), g);

                            for b in game.buyables.iter() {
                                if b.is_visible {
                                    image(&player, c.transform.trans(b.x, b.y), g);
                                }
                            }
                        });
                    }
                    "compare" => {
                        //println!("{:?}", game.player.basket);
                        window.draw_2d(&e, |c, g| {
                            clear([0.0, 0.0, 0.0, 1.0], g);

                            text::Text::new_color(color, 30).draw(
                                &"Let's see what you bought.",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(color, 30).draw(
                                &format!("You got {:?} right.", game.bought_correctly.join(", ")),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 300.0), g);

                            text::Text::new_color(color, 30).draw(
                                &format!("{:?} things are not there.", game.not_there),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 350.0), g);

                            text::Text::new_color(color, 30).draw(
                                &format!("But you got {:?} things that are not necessary.", game.bought_too_much),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 400.0), g);

                            text::Text::new_color(color, 30).draw(
                                "Press <space> to shrugg",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);
                        });

                    }

                    "resume" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 1.0, 1.0, 1.0], g);

                            talk_scene.draw(c.transform, g);
                            text::Text::new_color(color, 30).draw(
                                "Press <space> to shop again",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }

                    "status" => {
                        window.draw_2d(&e, |c, g| {
                            clear([0.0, 0.0, 1.0, 1.0], g);

                            text::Text::new_color(color, 30).draw(
                                &"Your energy",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(color, 30).draw(
                                &"Your relationship",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 300.0), g);

                            text::Text::new_color(color, 30).draw(
                                "Press <space> to go on",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);


                            let energie_total = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(color, energie_total, c.transform.trans(
                                50.0, 200.0).scale(10.0, 1.0), g);

                            let energie = rectangle::square(0.0, 0.0, 20.0);
                            rectangle([1.0, 1.0, 1.0, 1.0], energie, c.transform.trans(
                                50.0, 200.0).scale(game.energy, 1.0), g);

                            let rel_total = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(color, rel_total, c.transform.trans(
                                50.0, 300.0).scale(10.0, 1.0), g);

                            let rel = rectangle::square(0.0, 0.0, 20.0);
                            rectangle([1.0, 1.0, 1.0, 1.0], rel, c.transform.trans(
                                50.0, 300.0).scale(game.relationship, 1.0), g);
                        });
                    }
                    "game_over" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 0.0, 1.0, 1.0], g);

                            text::Text::new_color(color, 30).draw(
                                "I think we need to talk",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 280.0), g);

                            text::Text::new_color(color, 30).draw(
                                "Press <esc> to close",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    "win" => {
                        window.draw_2d(&e, |c, g| {
                            clear([0.0, 1.0, 0.0, 1.0], g);

                            text::Text::new_color(color, 30).draw(
                                "Perfect, thank you so much.",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 280.0), g);

                            text::Text::new_color(color, 30).draw(
                                "Press <esc> to close",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    _ => {}

                }

            }
            _ => {}
        }
    }
}
