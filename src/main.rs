use raylib::prelude::*;
use std::env;
use std::process;
use csv::ReaderBuilder;
// TODO: 
// - create simple interface that shows each quesion and reveals the answer once a button is clicked on 
// - create a system to mark difficulty / not show questions marked as easy

struct QuestionT {
    question: String,
    answer: String,
    known: bool,
}

struct Config {
    width: i32,
    height: i32,
    file_path: String,
    num_questions: u64,
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });


    let questions: Vec<QuestionT>;
    (questions, config.num_questions) = QuestionT::new(&config.file_path);  

    let (mut raylib_handle, raylib_thread) = raylib::init()
        .size(config.width, config.height)
        .title("Flashcard Reader")
        .build();

    raylib_handle.set_target_fps(60);
        
    run(&mut raylib_handle, raylib_thread, questions, config);
}



impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enugh arguments")
        }

        // let width = args[1].parse::<i32>().expect("Invalid width provided");
        // let height = args[2].parse::<i32>().expect("Invalid height provided");
        // let file_path = args[3].clone();
        
        let width = 1000;
        let height = 400;
        let file_path = args[1].clone();

        let num_questions: u64 = 0;

        Ok(Config {
            width,
            height,
            file_path,
            num_questions,
        })
    }
}

impl QuestionT {
    pub fn new(file_path: &str) -> (Vec<QuestionT>, u64) {
        let mut questions: Vec<QuestionT> = Vec::new();  
        let mut num_questions = 0;

        let mut rdr = ReaderBuilder::new()
            .delimiter(b';')
            .from_path(file_path)
            .unwrap();

        for result in rdr.records() {
            if let Ok(record) = result {
                if let Some(question) = Self::parse_record(record) {
                    questions.push(question);
                    num_questions += 1;
                }
            }

        }


        (questions, num_questions) 
    }

    fn parse_record(record: csv::StringRecord) -> Option<QuestionT> {
        // Assuming the CSV file has two columns: question, answer
        if record.len() == 2 {
            let question = String::from(record.get(0).unwrap_or(""));
            let answer = String::from(record.get(1).unwrap_or(""));
            let known = false;

            Some(QuestionT { question, answer, known })
        } else {
            None
        }
    }
}



fn run(raylib_handle: &mut RaylibHandle, raylib_thread: RaylibThread, questions: Vec<QuestionT>, config: Config) {
    let mut button_pressed = false;
    let mut show_answer = false;

    let mut i = 0;
    while !raylib_handle.window_should_close() {
        let mut d = raylib_handle.begin_drawing(&raylib_thread);
        d.clear_background(Color::RAYWHITE);
          
        // d.draw_text(&questions[i].question, (width - measure_text(&questions[i].question, 20)) / 2, height/2, 20, Color::BLACK);
        draw_wrapped_text_centered(
            &mut d,
            &questions[i].question,
            40,
            20,
            config.width - 80, // Max width for wrapping
            Color::BLACK,
        );

        if show_answer{
            draw_wrapped_text_centered(
                &mut d,
                &questions[i].answer,
                80,
                20,
                config.width - 80, // Max width for wrapping
                Color::BLACK,
            );
        }
        
        if d.is_key_down(consts::KeyboardKey::KEY_SPACE){
            if !button_pressed {
                show_answer = !show_answer;

                if !show_answer && i < questions.len() - 1 {
                    i += 1
                } else if !show_answer {
                    i = 0;
                }

                button_pressed = true;
                }        
        } else {
            button_pressed = false;
        }

    }

}

fn draw_wrapped_text_centered(
    d: &mut RaylibDrawHandle<'_>,
    text: &str,
    mut y: i32,
    font_size: i32,
    max_width: i32,
    color: Color,
) {
    let mut words = text.split_whitespace();
    let mut line = String::new();
    let mut line_width = 0;

    while let Some(word) = words.next() {
        let word_width = measure_text(word, font_size);
        if line_width + word_width < max_width {
            // Word fits on the current line
            line.push_str(word);
            line.push(' '); // Add space after the word
            line_width += word_width + measure_text(" ", font_size); // Include space width
        } else {
            // Word exceeds max_width, draw the current line and start a new line
            let x = (d.get_screen_width() - line_width) / 2;
            d.draw_text(&line, x - 15, y, font_size, color);
            y += (font_size as f32 * 1.2) as i32; // Increase line spacing
            line.clear();
            line.push_str(word);
            line.push(' '); // Add space after the word
            line_width = word_width + measure_text(" ", font_size); // Reset line width
        }
    }

    // Draw the last line
    if !line.is_empty() {
        let x = (d.get_screen_width() - line_width) / 2;
        d.draw_text(&line, x - 15, y, font_size, color);
    }
}
