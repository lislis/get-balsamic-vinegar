extern crate piston_window;
extern crate sprite;
extern crate find_folder;
extern crate ai_behavior;
extern crate rand;

mod player;
mod buyable;
mod game;

use std::rc::Rc;
use std::path::PathBuf;
use piston_window::*;
use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait
};
use game::Game;


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
        get_asset_path("pink-bubble-1.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());
    let tex2 = Rc::new(Texture::from_path(
        &mut window.factory,
        get_asset_path("green-bubble-1.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());
    let tex3 = Rc::new(Texture::from_path(
        &mut window.factory,
        get_asset_path("pink-bubble-2.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());
    let tex4 = Rc::new(Texture::from_path(
        &mut window.factory,
        get_asset_path("green-bubble-2.png"),
        Flip::None,
        &TextureSettings::new()).unwrap());

    let bubble_resume = Texture::from_path(
        &mut window.factory,
        &get_asset_path(&"pink-bubble-3.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let mut bubble = Sprite::from_texture(tex.clone());
    let mut bubble2 = Sprite::from_texture(tex2.clone());
    let mut bubble3 = Sprite::from_texture(tex3.clone());
    let mut bubble4 = Sprite::from_texture(tex4.clone());

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
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.7, 0.7))))
    ]);
    let talk_seq2 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(3.5),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.7, 0.7))))
    ]);
    let talk_seq3 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(5.0),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.7, 0.7))))
    ]);
    let talk_seq4 = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeOut(0.0)))),
        Wait(6.5),
        Action(Ease(EaseFunction::CubicOut, Box::new(FadeIn(0.5)))),
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.7, 0.7))))
    ]);
    talk_scene.run(bubble_id, &talk_seq);
    talk_scene.run(bubble_2, &talk_seq2);
    talk_scene.run(bubble_3, &talk_seq3);
    talk_scene.run(bubble_4, &talk_seq4);

    let pink_dark = [0.196, 0.160, 0.188, 1.0];
    let pink = [0.776, 0.109, 0.631, 1.0];
    let pink_light = [0.776, 0.631, 0.745, 1.0];
    let green_dark = [0.203, 0.243, 0.211, 1.0];
    let green = [0.109, 0.778, 0.203, 1.0];
    let green_light = [0.623, 0.776, 0.647, 1.0];

    let mut glyphs = Glyphs::new(
        get_asset_path("shpinscher-regular.ttf"),
        window.factory.clone()).unwrap();

    let mut game = Game::new(&"start");

    let player = Texture::from_path(
        &mut window.factory,
        &get_asset_path(&"cart.png"),
        Flip::None,
        &TextureSettings::new()
    ).unwrap();

    let tomato = Texture::from_path(
        &mut window.factory,
        &get_asset_path(&"tomato.png"),
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
                            clear(green_light, g);

                            text::Text::new_color(pink, 60).draw(
                                "Get balsamic vinegar",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(green_dark, 30).draw(
                                "Press <space> to start",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);
                        });
                    }
                    "talk" => {
                        window.draw_2d(&e, |c, g| {
                            clear(pink_dark, g);

                            talk_scene.draw(c.transform, g);
                            text::Text::new_color(green_light, 30).draw(
                                "Press <space> to go",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    "shop" => {
                        window.draw_2d(&e, |c, g| {
                            clear(green_light, g);

                            let bar = rectangle::square(0.0, 0.0, 100.0);
                            rectangle(green_dark, bar, c.transform.trans(
                                0.0, 0.0).scale(600.0, 1.0), g);

                            let bar2 = rectangle::square(0.0, 0.0, 100.0);
                            rectangle(green_dark, bar2, c.transform.trans(
                                0.0, 400.0).scale(600.0, 1.0), g);

                            image(&player, c.transform.trans(game.player.x, game.player.y), g);

                            for b in game.buyables.iter() {
                                if b.is_visible {
                                    image(&tomato, c.transform.trans(b.x, b.y), g);
                                }
                            }
                        });
                    }
                    "compare" => {
                        //println!("{:?}", game.player.basket);
                        window.draw_2d(&e, |c, g| {
                            clear(pink_light, g);

                            text::Text::new_color(pink_dark, 30).draw(
                                &"Let's see what you bought.",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 100.0), g);

                            text::Text::new_color(pink_dark, 30).draw(
                                &format!("You got {:?} right.", game.bought_correctly.join(", ")),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(pink_dark, 30).draw(
                                &format!("{:?} things are not there.", game.not_there),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 280.0), g);

                            text::Text::new_color(pink_dark, 30).draw(
                                &format!("But you got {:?} things that are not necessary.", game.bought_too_much),
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 360.0), g);

                            text::Text::new_color(green, 30).draw(
                                "Press <space> to shrugg",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);
                        });

                    }

                    "resume" => {
                        window.draw_2d(&e, |c, g| {
                            clear(pink_dark, g);

                            image(&bubble_resume, c.transform.trans(100.0, 200.0), g);
                            text::Text::new_color(green, 30).draw(
                                "Press <space> to shop again",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }

                    "status" => {
                        window.draw_2d(&e, |c, g| {
                            clear(green_light, g);

                            text::Text::new_color(green_dark, 30).draw(
                                &"Your energy",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 200.0), g);

                            text::Text::new_color(green_dark, 30).draw(
                                &"Your relationship",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 300.0), g);

                            text::Text::new_color(pink_dark, 30).draw(
                                "Press <space> to go on",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g);


                            let energie_total = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(pink_dark, energie_total, c.transform.trans(
                                50.0, 220.0).scale(10.0, 1.0), g);

                            let energie = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(pink, energie, c.transform.trans(
                                50.0, 220.0).scale(game.energy, 1.0), g);

                            let rel_total = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(pink_dark, rel_total, c.transform.trans(
                                50.0, 320.0).scale(10.0, 1.0), g);

                            let rel = rectangle::square(0.0, 0.0, 20.0);
                            rectangle(pink, rel, c.transform.trans(
                                50.0, 320.0).scale(game.relationship, 1.0), g);
                        });
                    }
                    "game_over" => {
                        window.draw_2d(&e, |c, g| {
                            clear(pink_dark, g);

                            text::Text::new_color(pink, 30).draw(
                                "I think we need to talk",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 280.0), g);

                            text::Text::new_color(green_light, 30).draw(
                                "Press <esc> to close",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 470.0), g)
                        });
                    }
                    "win" => {
                        window.draw_2d(&e, |c, g| {
                            clear(green_light, g);

                            text::Text::new_color(pink_dark, 30).draw(
                                "Perfect, thank you so much.",
                                &mut glyphs,
                                &c.draw_state,
                                c.transform.trans(30.0, 280.0), g);

                            text::Text::new_color(green_dark, 30).draw(
                                "Press <esc> to go eat",
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
