use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Position, Rect};
use ratatui::widgets::Widget;
use ratatui::widgets::{Bar, BarChart};

use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::zone::Zone;
use rand::random_range;

const LECONS: &str = include_str!("../../assets/lecons_agreg.txt");

#[derive(Default, Debug)]
pub struct Couplage {
    lecons: Vec<&'static str>,
    lecons_choosen: Vec<usize>,

    left: usize,
    right: usize,
}

impl Couplage {
    fn next_left(&mut self) {
        self.lecons_choosen[self.left] += 1;
        self.choose_lecons();
    }
    fn next_right(&mut self) {
        self.lecons_choosen[self.right] += 1;
        self.choose_lecons();
    }

    fn choose_lecons(&mut self) {
        self.left = random_range(0..self.lecons.len());
        self.right = random_range(0..self.lecons.len());
        while self.right == self.left {
            self.right = random_range(0..self.lecons.len());
        }
    }
}

fn parse_lecons(input: &str) -> impl Iterator<Item = &str> {
    input
        .split("\n") // Sépare chaque section délimitée par ---
        .map(str::trim) // Supprime les espaces autour des sections
        .filter(|s| !s.is_empty())
}

impl Ui for Couplage {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        if self.lecons.is_empty() {
            for lecon in parse_lecons(LECONS) {
                self.lecons.push(lecon);
                self.lecons_choosen.push(0);
            }
            self.choose_lecons();
        }
        if should_stop(events) {
            return Response::STOPPED;
        }
        if Zone::default()
            .min_area(Rect::new(0, 0, 40, 40))
            .ui(area, buf, events, mouse)
            .stopped()
        {
            return Response::NONE;
        }

        for event in events {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => self.next_right(),
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => self.next_left(),
                _ => {}
            }
        }

        let vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Fill(8),
            ])
            .split(area);
        let horizon = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(5),
                Constraint::Fill(1),
                Constraint::Fill(5),
                Constraint::Fill(1),
            ]);
        let left_arrow = horizon.split(vert[0])[1];
        let right_arrow = horizon.split(vert[0])[3];

        Zone::default()
            .text("->")
            .ui(right_arrow, buf, events, mouse);
        Zone::default()
            .text("<-")
            .ui(left_arrow, buf, events, mouse);

        let left_area = horizon.split(vert[1])[1];
        let right_area = horizon.split(vert[1])[3];

        if Zone::default()
            .bordered()
            .text(self.lecons[self.left])
            .ui(left_area, buf, events, mouse)
            .clicked()
        {
            self.next_left();
        };
        if Zone::default()
            .bordered()
            .text(self.lecons[self.right])
            .ui(right_area, buf, events, mouse)
            .clicked()
        {
            self.next_right();
        };

        let mut bar_vec = vec![];

        for (i, value) in self.lecons_choosen.iter().enumerate() {
            bar_vec.push(Bar::with_label(format!("{}", i + 1), *value as u64));
        }

        BarChart::new(bar_vec).bar_width(3).render(
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(self.lecons.len() as u16 * 4),
                    Constraint::Fill(1),
                ])
                .split(vert[2])[1],
            buf,
        );

        Response::NONE
    }
}
