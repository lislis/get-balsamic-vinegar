extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;

use std::rc::Rc;
use std::path::PathBuf;
use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait
};

struct Player {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    is_jumping: bool,
    max_height: f64
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 50.0,
            y: 300.0,
            w: 40.0,
            h: 30.0,
            is_jumping: false,
            max_height: 200.0
        }
    }
    pub fn update(&mut self, dt: f64) {
        println!("updating player");
    }
}

struct Game {
    state: &'static str,
    tiredness: f64,
    relationship: f64,
    player: Player
}

impl Game {
    pub fn new(param_state: &'static str) -> Game {
        Game {
            state: param_state,
            tiredness: 0.0,
            relationship: 10.0,
            player: Player::new()
        }
    }
    pub fn set_state(&mut self, state: &'static str) {
        self.state = state;
    }
    pub fn update(&mut self, dt: f64) {
        self.player.update(dt);
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

    let mut game = Game::new(&"talk");

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
                println!("{:?}", key);

                match game.state {
                    "talk" => {
                        if key == Key::Space {
                            game.set_state(&"shop");
                        }
                    }
                    "shop" => {

                    }
                    _ => {}

                }
            }

            Input::Update(args) => {
                match game.state {
                    "shop" => {
                        game.update(args.dt);
                    }
                    _ => {}
                }
            }

            Input::Render(_) => {

                match game.state {
                    "talk" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 1.0, 1.0, 1.0], g);

                            talk_scene.draw(c.transform, g);
                            text::Text::new_color(color, 30).draw(
                                "Press <space> to start",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    "shop" => {
                        window.draw_2d(&e, |c, g| {
                            clear([1.0, 0.0, 0.0, 1.0], g);
                            image(&player, c.transform, g);
                        });
                    }
                    _ => {}
                }

            }
            _ => {}
        }
    }
}
