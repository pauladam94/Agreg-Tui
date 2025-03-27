use crossterm::event::*;
use rand::random_range;
use ratatui::prelude::*;
use ratatui::widgets::*;
use std::collections::VecDeque;
use std::ops::Range;
use std::time::Instant;

use crate::ui::*;

const BASE_SPEED: u128 = 30;
const NEXT: [[i16; 2]; 4] = [[0, 1], [0, -1], [1, 0], [-1, 0]];
const BLOCK: [(u16, u16); 4] = [(0, 1), (1, 0), (0, 0), (1, 1)];

#[derive(Debug, Default, PartialEq)]
pub enum Direction {
    #[default]
    RIGHT,
    DOWN,
    UP,
    LEFT,
}

#[derive(Debug)]
pub struct Snake {
    pos: Vec<Position>,
    fruit: Position,
    dir: Direction,

    time: Instant,
    delta_time_millis: u128,

    hard_fruit: bool,
    show_dist: bool,
}

fn dif_pos(dir: &Direction) -> [i16; 2] {
    match dir {
        Direction::RIGHT => [1, 0],
        Direction::LEFT => [-1, 0],
        Direction::DOWN => [0, 1],
        Direction::UP => [0, -1],
    }
}

/// Compute `u + i` so that it is in `range`
fn add_modulo_range(u: u16, range: Range<u16>, i: i16) -> u16 {
    (u as i16 - range.start as i16 + i)
        .rem_euclid(range.end as i16 - range.start as i16) as u16
        + range.start
}

fn modulo_range(range: Range<u16>, i: i16) -> u16 {
    (i - range.start as i16).rem_euclid(range.end as i16 - range.start as i16)
        as u16
        + range.start
}

fn modulo_rect(rect: Rect, pos: &Position) -> Position {
    Position::new(
        modulo_range(rect.left()..rect.right(), pos.x as i16),
        modulo_range(rect.top()..rect.bottom(), pos.y as i16),
    )
}

impl Snake {
    pub fn new() -> Self {
        Self {
            pos: vec![Position::new(0, 0)],
            fruit: Position::new(10, 10),
            time: Instant::now(),
            delta_time_millis: BASE_SPEED,
            dir: Direction::RIGHT,
            hard_fruit: true,
            show_dist: false,
        }
    }
    fn update_delta_time(&mut self) {
        match self.dir {
            Direction::RIGHT | Direction::LEFT => {
                self.delta_time_millis = BASE_SPEED;
            }
            Direction::UP | Direction::DOWN => {
                self.delta_time_millis = BASE_SPEED * 2;
            }
        }
    }
    fn move_forward(&mut self, area: Rect) {
        self.move_body();
        self.move_head(area);
    }
    fn move_head(&mut self, area: Rect) {
        let dif = dif_pos(&self.dir);
        self.pos[0].x = add_modulo_range(
            self.pos[0].x,
            area.left() + 1..area.right() - 1,
            dif[0],
        );
        self.pos[0].y = add_modulo_range(
            self.pos[0].y,
            area.top() + 1..area.bottom() - 1,
            dif[1],
        );
    }
    fn move_body(&mut self) {
        for i in (0..self.pos.len() - 1).rev() {
            self.pos[i + 1] = self.pos[i];
        }
    }
    fn push_one_more(&mut self, area: Rect) {
        let new_pos = *self.pos.last().unwrap();
        self.move_forward(area);
        self.pos.push(new_pos);
    }
    fn head_on_fruit(&self) -> bool {
        for (x, y) in BLOCK {
            if *self.pos.first().unwrap()
                == Position::new(self.fruit.x + x, self.fruit.y + y)
            {
                return true;
            }
        }
        false
    }
    fn head_on_itself(&self) -> bool {
        if let Some((first, elem)) = self.pos.split_first() {
            elem.contains(first)
        } else {
            false
        }
    }

    pub fn tick(&mut self, area: Rect) -> Response {
        if self.time.elapsed().as_millis() >= self.delta_time_millis {
            self.time = Instant::now();
            self.move_forward(area);

            if self.head_on_fruit() {
                self.push_one_more(area);
                self.fruit = if self.hard_fruit {
                    let d = self.compute_distance(area);
                    let mut max_pos = Position::new(0, 0);
                    let mut max = 0;

                    for i in 0..d.len() {
                        for j in 0..d[0].len() {
                            if let Some(dist) = d[i][j] {
                                if dist > max {
                                    max = dist;
                                    max_pos.x = i as u16;
                                    max_pos.y = j as u16;
                                }
                            }
                        }
                    }
                    max_pos
                } else {
                    Position::new(
                        random_range(1..area.width - 2),
                        random_range(1..area.height - 2),
                    )
                };
                self.hard_fruit = !self.hard_fruit;
            }
            if self.head_on_itself() {
                return Response::STOPPED;
            }
        }

        Response::NONE
    }

    fn handle_event(&mut self, area: Rect, event: &Event) -> Response {
        match &event {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                return self.handle_key_event(area, key_event);
            }
            _ => {}
        };
        Response::NONE
    }
    fn handle_key_event(
        &mut self,
        area: Rect,
        key_event: &KeyEvent,
    ) -> Response {
        match key_event.code {
            KeyCode::Left | KeyCode::Char('h') => {
                if self.dir != Direction::RIGHT {
                    self.dir = Direction::LEFT
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.dir != Direction::DOWN {
                    self.dir = Direction::UP
                }
            }
            KeyCode::Right | KeyCode::Char('l') => {
                if self.dir != Direction::LEFT {
                    self.dir = Direction::RIGHT
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.dir != Direction::UP {
                    self.dir = Direction::DOWN
                }
            }
            KeyCode::Char('d') => self.show_dist = !self.show_dist,
            KeyCode::Char('q') => return Response::STOPPED,
            KeyCode::Char('a') => {
                self.push_one_more(area);
            }
            _ => {}
        }
        self.update_delta_time();
        Response::NONE
    }

    fn compute_distance(&self, area: Rect) -> Vec<Vec<Option<u16>>> {
        let mut d: Vec<Vec<Option<u16>>> = vec![];
        for i in 0..(area.width as usize) {
            d.push(vec![]);
            for _ in 0..(area.height as usize) {
                d[i].push(None);
            }
        }
        let mut todo: VecDeque<Position> = VecDeque::from([self.pos[0]]);
        d[modulo_range(area.left()..area.right(), self.pos[0].x as i16)
            as usize][self.pos[0].y as usize] = Some(0);
        while !todo.is_empty() {
            let current = todo.pop_front().unwrap();
            for [x, y] in NEXT {
                let next_pos = Position::new(
                    add_modulo_range(
                        current.x,
                        area.left() + 1..area.right() - 1,
                        x,
                    ),
                    add_modulo_range(
                        current.y,
                        area.top() + 1..area.bottom() - 1,
                        y,
                    ),
                );

                if d[next_pos.x as usize][next_pos.y as usize].is_some()
                    || self.pos.contains(&next_pos)
                {
                    continue;
                }
                d[next_pos.x as usize][next_pos.y as usize] = Some(
                    d[current.x as usize][current.y as usize].unwrap() + 1,
                );
                todo.push_back(next_pos);
            }
        }
        d
    }

    fn render_distance(
        &self,
        area: Rect,
        dist: Vec<Vec<Option<u16>>>,
        buf: &mut Buffer,
    ) {
        for i in 0..(area.width as usize) {
            for j in 0..(area.height as usize) {
                if (i + j).rem_euclid(2) == 0 {
                    continue;
                }
                if let Some(cell) =
                    buf.cell_mut(Position::new(i as u16, j as u16))
                {
                    let s = match dist[i][j] {
                        None => "∞",
                        Some(i) => {
                            if i >= 10 {
                                "9"
                            } else {
                                &format!("{}", i)
                            }
                        }
                    };
                    cell.set_symbol(s);
                }
            }
        }
    }
}

impl Ui for Snake {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        _mouse: Position,
    ) -> Response {
        self.pos.iter_mut().for_each(|p| {
            *p = modulo_rect(area, p);
        });
        self.fruit = modulo_rect(area, &self.fruit);
        if self.pos.is_empty() {
            self.pos.push(Position::new(area.left(), area.top()));
        }

        if self.tick(area).stopped() {
            return Response::STOPPED;
        }
        for event in events {
            if self.handle_event(area, event).stopped() {
                return Response::STOPPED;
            }
        }
        // render
        for p in &self.pos {
            if let Some(cell) = buf.cell_mut(*p) {
                cell.set_symbol("□")
                    .set_style(Style::default().fg(Color::Red));
            }
        }
        if self.show_dist {
            self.render_distance(area, self.compute_distance(area), buf);
        }

        for (x, y) in BLOCK {
            if let Some(cell) =
                buf.cell_mut(Position::new(self.fruit.x + x, self.fruit.y + y))
            {
                cell.set_symbol("█")
                    .set_style(Style::default().fg(Color::Green));
            }
        }

        Text::from(format!("Score {}", self.pos.len()))
            .render(Rect::new(2, area.bottom() - 2, 10, 10), buf);

        Block::bordered()
            .border_type(BorderType::Rounded)
            .render(area, buf);

        Response::NONE
    }
}
