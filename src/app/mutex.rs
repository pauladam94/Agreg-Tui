use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::text_edit::TextEdit;
use crossterm::event::Event;
use ratatui::layout::Margin;
use ratatui::prelude::{Buffer, Position, Rect};

#[derive(Default, Debug)]
pub struct Mutex {
    code: Vec<String>,
    focused: bool,
}

impl Ui for Mutex {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        if should_stop(events) {
            return Response::STOPPED;
        }
        if self.code.is_empty() {
            self.code.push("test".into());
        }

        TextEdit::new(&mut self.code[0], &mut self.focused).ui(
            area.inner(Margin::new(1, 1)),
            buf,
            events,
            mouse,
        );
        Response::NONE
    }
}
