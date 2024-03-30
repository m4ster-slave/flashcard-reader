use flashcard_reader::config::Config;
use flashcard_reader::draw;
use flashcard_reader::question;
use raylib::prelude::*;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut questions: Vec<question::QuestionT>;
    questions = question::QuestionT::new(&config.file_path);

    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(config.width, config.height)
        .title("Flashcard Reader")
        .build();

    raylib_handle.set_target_fps(60);

    run(&mut raylib_handle, raylib_thread, &mut questions, config);
}

fn run(
    raylib_handle: &mut RaylibHandle,
    raylib_thread: RaylibThread,
    questions: &mut Vec<question::QuestionT>,
    config: Config,
) {
    let mut show_answer = false;
    let mut i = 0;
    let mut all_easy = false;
    let color_darkred: Color = Color::from_hex("8b0000").unwrap();

    while !raylib_handle.window_should_close() {
        let mouse_point = raylib_handle.get_mouse_position();

        let mut d = raylib_handle.begin_drawing(&raylib_thread);

        //clearing background
        d.clear_background(Color::RAYWHITE);

        // Draw text
        draw::draw_wrapped_text_centered(
            &mut d,
            &questions[i].question,
            40,
            20,
            config.width - 80, // Max width for wrapping
            Color::BLACK,
        );
        if show_answer {
            draw::draw_wrapped_text_centered(
                &mut d,
                &questions[i].answer,
                80,
                20,
                config.width - 80, // Max width for wrapping
                Color::BLACK,
            );
        }

        // Draw buttons
        let easy_rec = Rectangle {
            x: 150.0,
            y: 350.0,
            width: 100.0,
            height: 35.0,
        };

        let next_rec = Rectangle {
            x: 425.0,
            y: 350.0,
            width: 150.0,
            height: 35.0,
        };

        let hard_rec = Rectangle {
            x: 750.0,
            y: 350.0,
            width: 100.0,
            height: 35.0,
        };

        if easy_rec.check_collision_point_rec(mouse_point) {
            d.draw_rectangle_rounded(easy_rec, 0.85, 0, color_darkred);
            if d.is_mouse_button_released(consts::MouseButton::MOUSE_LEFT_BUTTON) {
                i = question::QuestionT::next(questions, i).unwrap();
                show_answer = false;
                continue;
            };
        } else {
            d.draw_rectangle_rounded(easy_rec, 0.85, 0, Color::RED);
        }

        if next_rec.check_collision_point_rec(mouse_point) {
            d.draw_rectangle_rounded(next_rec, 0.85, 0, Color::DARKBLUE);
            if d.is_mouse_button_released(consts::MouseButton::MOUSE_LEFT_BUTTON) {
                show_answer = true
            };
        } else {
            d.draw_rectangle_rounded(next_rec, 0.85, 0, Color::BLUE);
        }

        if hard_rec.check_collision_point_rec(mouse_point) {
            d.draw_rectangle_rounded(hard_rec, 0.85, 0, Color::DARKGREEN);
            if d.is_mouse_button_released(consts::MouseButton::MOUSE_LEFT_BUTTON) {
                questions[i].mark_as_easy();
                i = question::QuestionT::next(questions, i).unwrap();
                all_easy = question::QuestionT::check_if_all_questions_easy(questions);
                show_answer = false;
                continue;
            };
        } else {
            d.draw_rectangle_rounded(hard_rec, 0.85, 0, Color::GREEN);
        }

        // finished prompt
        if all_easy {
            let prompt = "All questions answered, again? [Y/N]";
            d.draw_rectangle(0, 0, config.width, config.height, Color::LIGHTGRAY);
            println!("{}", (config.width - measure_text(prompt, 10)) / 2);
            d.draw_text(
                prompt,
                (config.width - measure_text(prompt, 30)) / 2,
                200,
                30,
                Color::RED,
            );
            if d.is_key_pressed(consts::KeyboardKey::KEY_Y) {
                for question in &mut *questions {
                    question.is_easy = false;
                }
                all_easy = false;
                continue;
            } else if d.is_key_pressed(consts::KeyboardKey::KEY_N) {
                break;
            }
        }
    }
}
