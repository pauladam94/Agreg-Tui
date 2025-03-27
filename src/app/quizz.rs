use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::zone::Zone;
use crossterm::event::{Event, KeyCode, KeyEvent};
use rand::random_range;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Position, Rect};

const QUIZZ_TEXT: &'static str = include_str!("../../assets/quizz.txt");

#[derive(Debug, Default, PartialEq)]
enum QuizState {
    #[default]
    Question,
    Answer,
}

#[derive(Debug, Default)]
pub struct Quiz {
    questions: Vec<(&'static str, &'static str)>,
    index: usize,

    question_done: Vec<usize>,
    state: QuizState,
}

fn parse_questions(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input
        .split("---") // Sépare chaque section délimitée par ---
        .map(str::trim) // Supprime les espaces autour des sections
        .filter(|s| !s.is_empty())
        .filter_map(|section| {
            let mut parts = section.splitn(2, "\n\n"); // Split into two parts: question & answer
            let question = parts.next()?.trim();
            let answer = parts.next()?.trim();
            Some((question, answer))
        })
}

impl Quiz {
    fn next(&mut self) {
        match self.state {
            // demand the answer
            QuizState::Question => {
                self.state = QuizState::Answer;
            }
            QuizState::Answer => {
                self.state = QuizState::Question;
                self.question_done.push(self.index);
                while self.question_done.contains(&self.index) {
                    self.index = random_range(0..self.questions.len());
                }
            }
        }
    }
}

impl Ui for Quiz {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        let shortcut_area = Layout::default()
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(area)[1];
        Zone::default().text("| [->] Next |").ui(
            shortcut_area,
            buf,
            events,
            mouse,
        );

        if self.questions.is_empty() {
            for (question, answer) in parse_questions(QUIZZ_TEXT) {
                self.questions.push((question, answer))
            }
        }
        if should_stop(events) {
            return Response::STOPPED;
        }
        for event in events {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => self.next(),
                _ => {}
            }
        }
        let vert_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Fill(2),
                Constraint::Fill(1),
            ])
            .split(area);
        let question_area = vert_layout[1];
        let answer_area = vert_layout[2];

        let horizontal_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(4),
                Constraint::Fill(1),
            ]);
        let (question_txt, answer_txt) = self.questions[self.index];
        Zone::default().text(question_txt).ui(
            horizontal_layout.split(question_area)[1],
            buf,
            events,
            mouse,
        );

        if self.state == QuizState::Answer {
            Zone::default().text(answer_txt).ui(
                horizontal_layout.split(answer_area)[1],
                buf,
                events,
                mouse,
            );
        }

        // if Zone::default()
        //     .text("First Question")
        //     .bg(Color::Green)
        //     .ui(layout, buf, events, mouse)
        //     .clicked()
        // {
        // }
        Response::NONE
    }
}
