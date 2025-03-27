use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::{Position, Rect};

#[derive(Debug, Default)]
pub struct Response {
    stopped: bool,
    clicked: bool,
    hovered: bool,
}

impl Response {
    pub const NONE: Self = Self {
        stopped: false,
        clicked: false,
        hovered: false,
    };
    pub const STOPPED: Self = Self {
        stopped: true,
        clicked: false,
        hovered: false,
    };
    pub const HOVER: Self = Self {
        stopped: true,
        clicked: false,
        hovered: true,
    };
    pub fn stopped(self) -> bool {
        self.stopped
    }
    pub fn stop(mut self) -> Self {
        self.stopped = true;
        self
    }
    pub fn clicked(&self) -> bool {
        self.clicked
    }
    pub fn click(mut self) -> Self {
        self.clicked = true;
        self
    }
    pub fn hovered(&self) -> bool {
        self.hovered
    }
    pub fn hover(mut self) -> Self {
        self.hovered = true;
        self
    }
}

impl std::ops::AddAssign for Response {
    fn add_assign(&mut self, rhs: Self) {
        self.stopped |= rhs.stopped;
        self.clicked |= rhs.clicked;
        self.hovered |= rhs.hovered;
    }
}

impl std::ops::Add for Response {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stopped: self.stopped | rhs.stopped,
            clicked: self.clicked | rhs.clicked,
            hovered: self.hovered | rhs.hovered,
        }
    }
}

pub trait Ui {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response;
}
