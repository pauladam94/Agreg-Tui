mod couplage;
mod games;
mod hirschberg;
mod mutex;
mod quizz;

use self::couplage::Couplage;
use self::games::Games;
use self::hirschberg::Hirschberg;
use self::mutex::Mutex;
use self::quizz::Quiz;
use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::buttons_list::ButtonsList;
use crossterm::event::*;
use ratatui::buffer::Buffer;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

#[derive(Debug)]
pub struct App {
    hirschberg: Hirschberg,
    quizz: Quiz,
    games: Games,
    state: State,
    mutex: Mutex,
    couplage: Couplage,
}

#[derive(Debug)]
enum State {
    Menu,
    Games,
    Hirschberg,
    Quizz,
    Mutex,
    Couplage,
}

impl App {
    pub fn new() -> Self {
        Self {
            hirschberg: Hirschberg::new(),
            state: State::Menu,
            games: Games::new(),
            quizz: Quiz::default(),
            mutex: Mutex::default(),
            couplage: Couplage::default(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl Ui for App {
    fn ui(
        &mut self,
        mut area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        Block::bordered().render(area, buf);
        area = area.inner(Margin::new(1, 1));
        match self.state {
            State::Menu => {
                if should_stop(events) {
                    return Response::STOPPED;
                }
                let mut index = 0;
                if ButtonsList::new(
                    &mut index,
                    vec![
                        "🎮 Games 🎮",
                        "Hirschberg",
                        "Quizz",
                        "💻Mutex💻",
                        "📊Couplage📊",
                    ],
                )
                .ui(area, buf, events, mouse)
                .clicked()
                {
                    match index {
                        0 => self.state = State::Games,
                        1 => self.state = State::Hirschberg,
                        2 => self.state = State::Quizz,
                        3 => self.state = State::Mutex,
                        4 => self.state = State::Couplage,
                        _ => {}
                    }
                }
                Response::NONE
            }
            State::Games => {
                if self.games.ui(area, buf, events, mouse).stopped() {
                    self.state = State::Menu;
                }
                Response::NONE
            }
            State::Hirschberg => {
                if self.hirschberg.ui(area, buf, events, mouse).stopped() {
                    self.state = State::Menu;
                }
                Response::NONE
            }
            State::Quizz => {
                if self.quizz.ui(area, buf, events, mouse).stopped() {
                    self.state = State::Menu;
                }
                Response::NONE
            }
            State::Mutex => {
                if self.mutex.ui(area, buf, events, mouse).stopped() {
                    self.state = State::Menu;
                }
                Response::NONE
            }
            State::Couplage => {
                if self.couplage.ui(area, buf, events, mouse).stopped() {
                    self.state = State::Menu;
                }
                Response::NONE
            }
        }
    }
}
