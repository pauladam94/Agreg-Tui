use self::zone::Zone;
use crate::ui::{Response, Ui};
use crate::widgets::*;
use crossterm::event::Event;
use ratatui::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

pub struct ButtonsList<'a> {
    names: Vec<&'a str>,
    focus: &'a mut usize,
}

impl<'a> ButtonsList<'a> {
    pub fn new(focus: &'a mut usize, names: Vec<&'a str>) -> Self {
        Self { names, focus }
    }
}

impl Ui for ButtonsList<'_> {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        let mut rep = Response::NONE;
        let mut button_constraints = vec![Constraint::Fill(1)];
        for _ in 0..self.names.len() {
            button_constraints.push(Constraint::Length(3));
        }
        button_constraints.push(Constraint::Fill(1));
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(button_constraints)
            .split(area);

        for (i, text) in self.names.iter().enumerate() {
            let width = text.graphemes(true).count() as u16 + 10;
            let button_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(width + width % 2),
                    Constraint::Fill(1),
                ])
                .split(layout[i + 1]);
            let button_area = Rect::new(
                button_layout[1].x,
                button_layout[1].y,
                button_layout[1].width,
                button_layout[1].height,
            );
            if Zone::default()
                .bordered()
                .text(text)
                .ui(button_area, buf, events, mouse)
                .clicked()
            {
                *self.focus = i;
                rep = rep.click();
            }
        }
        rep
    }
}
