extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;

use std::rc::Rc;
use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    While
};

struct Game {
    state: String
}

impl Game {
    pub fn new(param_state:&str) -> Game {
        Game {
            state: String::from(param_state)
        }
    }

    pub fn set_state(&mut self, state: &str) {
        self.state = String::from(state);
    }
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

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let bubble_id;
    let bubble_2;
    let bubble_3;
    let bubble_4;
    let mut talk_scene = Scene::new();
    let tex = Rc::new(Texture::from_path(
        &mut window.factory,
        assets.join("bubble.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());

    let tex2 = Rc::new(Texture::from_path(
        &mut window.factory,
        assets.join("bubble.png"),
        Flip::Horizontal,
        &TextureSettings::new()).unwrap());

    let mut game = Game::new(&"talk");

    let mut bubble = Sprite::from_texture(tex.clone());
    let mut bubble2 = Sprite::from_texture(tex2.clone());
    bubble.set_position(200.0, 100.0);
    bubble2.set_position(400.0, 200.0);
    let mut bubble3 = Sprite::from_texture(tex.clone());
    let mut bubble4 = Sprite::from_texture(tex2.clone());
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
    let font = assets.join("shpinscher-regular.ttf");
    let mut glyphs = Glyphs::new(font, window.factory.clone()).unwrap();

    while let Some(e) = window.next() {

        if game.state == "talk" {
            talk_scene.event(&e);
        }

        match e {

            Input::Release(Button::Keyboard(key)) => {
                println!("{:?}", key);
            }

            Input::Update(args) => {

            }

            Input::Render(_) => {
                window.draw_2d(&e, |c, g| {
                    clear([1.0, 1.0, 1.0, 1.0], g);

                    if game.state == "talk" {
                        talk_scene.draw(c.transform, g);
                        text::Text::new_color(color, 30).draw(
                            "Press <space> to start",
                            &mut glyphs,
                            &c.draw_state,
                            c.transform.trans(30.0, 470.0), g)
                    }
                });
            }
            _ => {}
        }
    }
}
