use crate::ui::{Response, Ui};
use crate::widgets::zone::Zone;
use crossterm::event::*;
use ratatui::prelude::*;
use ratatui::widgets::*;

use self::block::Block;

#[derive(Default, Debug)]
pub struct Hirschberg {
    arr: Vec<Vec<Option<u16>>>,
    s1: String,
    s2: String,
    i: usize,
    j: usize,
}

impl Hirschberg {
    pub fn new() -> Self {
        let mut res = Self {
            arr: vec![],
            s1: "bonjour".to_string(),
            s2: "bonjoournoo".to_string(),
            i: 0,
            j: 0,
        };

        for i in 0..res.s1.len() {
            res.arr.push(vec![]);
            for _ in 0..res.s2.len() {
                res.arr[i].push(None);
            }
        }
        res.init_tab();

        res
    }

    fn init_tab(&mut self) {
        for j in 0..self.s2.len() {
            self.arr[0][j] = Some(j as u16);
        }
        for i in 0..self.s1.len() {
            self.arr[i][0] = Some(i as u16);
        }
    }

    fn decr_i_j(&mut self) {
        if self.j == 0 {
            self.j = self.s2.len() - 1;
            self.i -= 1;
        } else {
            self.j -= 1;
        }
    }
    fn incr_i_j(&mut self) {
        if self.j == self.s2.len() - 1 {
            self.j = 0;
            self.i += 1;
        } else {
            self.j += 1;
        }
    }

    fn do_one_step(&mut self) {
        self.arr[self.i][self.j] = Some(2);
        self.incr_i_j();
    }
}

impl Ui for Hirschberg {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        for event in events {
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    ..
                }) => return Response::STOPPED,
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => {
                    self.do_one_step();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    self.decr_i_j();
                }
                _ => {}
            }
        }

        if Zone::default()
            .min_area(Rect::new(
                0,
                0,
                self.s1.len() as u16 * 2,
                self.s2.len() as u16 * 2,
            ))
            .ui(area, buf, events, mouse)
            .stopped()
        {
            return Response::NONE;
        }

        if area.width as usize <= self.s1.len() * 2
            || area.height as usize <= self.s2.len() * 2
        {
            Text::from(format!(
                "Should be more than {}x{}",
                self.s1.len() * 2,
                self.s2.len() * 2
            ))
            .render(Rect::new(10, 10, 100, 1), buf);
            return Response::NONE;
        }

        Block::bordered().render(
            Rect::new(
                area.left(),
                area.top(),
                self.s1.len() as u16 * 2 + 3,
                self.s2.len() as u16 * 2 + 3,
            ),
            buf,
        );

        for i in 0..self.arr.len() {
            for j in 0..self.arr[0].len() {
                let x = 2 * (i as u16 + area.left());
                let y = 2 * (j as u16 + area.top());
                match self.arr[i][j] {
                    None => {
                        if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                            cell.set_symbol("∞");
                        }
                    }
                    Some(val) => {
                        if let Some(cell) = buf.cell_mut(Position::new(x, y)) {
                            cell.set_symbol(&format!("{}", val));
                        }
                        Block::bordered()
                            .render(Rect::new(x - 1, y - 1, 3, 3), buf);
                    }
                }
            }
        }
        let xi = 2 * (self.i as u16 + area.left()) - 1;
        let yj = 2 * (self.j as u16 + area.top()) - 1;
        Block::bordered()
            .border_type(BorderType::Double)
            .render(Rect::new(xi, yj, 3, 3), buf);

        Response::NONE
    }
}
