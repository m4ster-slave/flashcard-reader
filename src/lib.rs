pub mod config {
    pub struct Config {
        pub width: i32,
        pub height: i32,
        pub file_paths: Vec<String>,
    }

    impl Config {
        pub fn build(args: &[String]) -> Result<Config, &'static str> {
            if args.len() < 2 {
                return Err("need atleast one csv file");
            }

            let width = 1000;
            let height = 400;

            let mut file_paths: Vec<String> = Vec::new();
            for i in 1..(args.len()) {
                file_paths.push(args[i].clone());
            }

            Ok(Config {
                width,
                height,
                file_paths,
            })
        }
    }
}

pub mod question {
    pub use csv::ReaderBuilder;

    pub struct QuestionT {
        pub question: String,
        pub answer: String,
        pub is_easy: bool,
    }

    impl QuestionT {
        pub fn new(file_paths: &Vec<String>) -> Vec<QuestionT> {
            let mut questions: Vec<QuestionT> = Vec::new();

            for path in file_paths {
                let mut rdr = ReaderBuilder::new()
                    .delimiter(b';')
                    .from_path(path)
                    .unwrap();

                for result in rdr.records() {
                    if let Ok(record) = result {
                        if let Some(question) = Self::parse_record(record) {
                            questions.push(question);
                        }
                    }
                }
                }

            questions
        }

        pub fn mark_as_easy(&mut self) {
            self.is_easy = true;
        }

        pub fn next(questions: &mut Vec<QuestionT>, current_index: usize) -> Option<usize> {
            for (index, question) in questions.iter().enumerate().skip(current_index + 1) {
                if !question.is_easy {
                    return Some(index);
                }
            }
            Some(0)
        }

        pub fn check_if_all_questions_easy(questions: &Vec<QuestionT>) -> bool {
            for question in questions {
                if question.is_easy == false {
                    return false;
                };
            }

            return true;
        }

        fn parse_record(record: csv::StringRecord) -> Option<QuestionT> {
            // Assuming the CSV file has two columns: question, answer
            if record.len() == 2 {
                let question = String::from(record.get(0).unwrap_or(""));
                let answer = String::from(record.get(1).unwrap_or(""));
                let is_easy = false;

                Some(QuestionT {
                    question,
                    answer,
                    is_easy,
                })
            } else {
                None
            }
        }
    }
}

pub mod draw {
    use raylib::prelude::*;

    pub fn draw_wrapped_text_centered(
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
}
