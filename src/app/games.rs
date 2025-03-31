pub mod snake;

use self::buttons_list::ButtonsList;
use self::snake::Snake;
use self::zone::Zone;
use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::*;
use crossterm::event::Event;
use crossterm::event::*;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::BorderType;

#[derive(Default, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Snake,
    Test,
}

#[derive(Default, Debug)]
pub struct Games {
    state: GameState,

    snake: Vec<Snake>,
}

impl Games {
    pub fn new() -> Self {
        Self {
            state: GameState::default(),

            snake: vec![Snake::new(), Snake::new(), Snake::new(), Snake::new()],
        }
    }
}

impl Ui for Games {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: ratatui::prelude::Position,
    ) -> Response {
        match self.state {
            GameState::Menu => {
                if should_stop(events) {
                    return Response::STOPPED;
                }
                let mut index: usize = 0;
                if ButtonsList::new(&mut index, vec!["Snake", "Test"])
                    .ui(area, buf, events, mouse)
                    .clicked()
                {
                    match index {
                        0 => self.state = GameState::Snake,
                        1 => self.state = GameState::Test,
                        _ => {}
                    }
                }

                Response::NONE
            }
            GameState::Snake => {
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(vec![
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(area);
                if self.snake[0].ui(layout[0], buf, events, mouse).stopped()
                    || self.snake[1].ui(layout[1], buf, events, mouse).stopped()
                {
                    self.state = GameState::Menu;
                };
                Response::NONE
            }
            GameState::Test => {
                if should_stop(events) {
                    self.state = GameState::Menu;
                }

                Zone::new().bordered().ui(
                    Rect::new(4, 4, 4, 4).offset(Offset::new(
                        area.left() as i32,
                        area.top() as i32,
                    )),
                    buf,
                    events,
                    mouse,
                );

                Zone::new().bordered().border_type(BorderType::Thick).ui(
                    Rect::new(6, 6, 3, 3).offset(Offset::new(
                        area.left() as i32,
                        area.top() as i32,
                    )),
                    buf,
                    events,
                    mouse,
                );
                Zone::new().bordered().border_type(BorderType::Rounded).ui(
                    Rect::new(10, 10, 5, 5).offset(Offset::new(
                        area.left() as i32,
                        area.top() as i32,
                    )),
                    buf,
                    events,
                    mouse,
                );

                Zone::new().bordered().border_type(BorderType::Double).ui(
                    Rect::new(20, 30, 10, 10).offset(Offset::new(
                        area.left() as i32,
                        area.top() as i32,
                    )),
                    buf,
                    events,
                    mouse,
                );

                Response::NONE
            }
        }
    }
}
