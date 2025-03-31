use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::prelude::{Buffer, Position, Rect};

use crate::ui::{Response, Ui};

use super::zone::Zone;

pub struct TextEdit<'a> {
    text: &'a mut String,
    focused: &'a mut bool,
}

impl<'a> TextEdit<'a> {
    pub fn new(text: &'a mut String, focused: &'a mut bool) -> Self {
        Self { text, focused }
    }
}

impl Ui for TextEdit<'_> {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        // not perfect right now
        // but ok
        *self.focused = area.contains(mouse);

        Zone::default().text(self.text).ui(area, buf, events, mouse);
        if !*self.focused {
            return Response::NONE;
        }

        for event in events {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    self.text.push(*c);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    self.text.push('\n');
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Delete,
                    ..
                }) => {
                    self.text.pop();
                }
                _ => {}
            }
        }
        Response::NONE
    }
}
