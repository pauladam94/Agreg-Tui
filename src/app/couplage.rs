use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::{Buffer, Position, Rect};
use ratatui::text::{Line, Text};
use ratatui::widgets::Bar;
use ratatui::widgets::{Paragraph, Widget};

use crate::event::should_stop;
use crate::ui::{Response, Ui};
use crate::widgets::zone::Zone;
use rand::random_range;

// Representation of a an Order for elements
// It must be transitive.
#[derive(Debug, Default)]
struct Order {
    // parent[b] = Some(b) inside edges means a >= b
    parent: Vec<Option<usize>>,
    incoherent: bool,
}

impl Order {
    fn new() -> Self {
        let mut parent = vec![];
        for _ in 0..NB_LECONS + 1 {
            parent.push(None);
        }
        Self {
            parent,
            ..Default::default()
        }
    }
    fn childs(&self, parent: usize) -> Vec<usize> {
        let mut res = vec![];
        for (i, parent_i) in self.parent.iter().enumerate() {
            if let Some(p) = parent_i {
                if *p == parent {
                    res.push(i)
                }
            }
        }
        res
    }

    // Check that lhs >= rhs in the order
    // Since we have a partial order.
    // This function returns :
    // - None when no order exists.
    // - Some(true) if parent is a parent of child
    // - Some(false) if the opposite is true
    fn gt(&self, lhs: usize, rhs: usize) -> Option<bool> {
        if self.is_child(rhs, lhs) {
            return Some(true);
        }
        if self.is_child(lhs, rhs) {
            return Some(false);
        }
        None
    }
    // Check that lhs <= rhs in the order
    // Since we have a partial order.
    // This function returns
    // - None when no order exists.
    // - Some(true) if parent is a parent of child
    // - Some(false) if the opposite is true
    fn is_child(&self, child: usize, parent: usize) -> bool {
        if self.parent[child] == Some(parent) {
            return true;
        }

        for child_of_parent in self.childs(parent) {
            if self.is_child(child, child_of_parent) {
                return true;
            }
        }
        false
    }
    // Add the relation bigger >= smaller
    fn add_relation(&mut self, bigger: usize, smaller: usize) {
        while self.parent.len() <= bigger || self.parent.len() <= smaller {
            self.parent.push(None);
        }
        match self.gt(bigger, smaller) {
            None => self.parent[smaller] = Some(bigger),
            Some(v) => {
                if v {
                    self.incoherent = true;
                }
            }
        }
    }

    fn draw_tree_from(
        &self,
        buf: &mut Buffer,
        from: usize,
        pos: &mut Position,
        is_left: bool,
        pref: String,
    ) {
        /*
        ┌─────┐
        │ int │
        ├─────┤
        │ 2   │
        └─────┘
        */
        let txt = if is_left {
            format!("{}├─{from}", &pref)
        } else {
            format!("{}└─{from}", &pref)
        };
        Text::from(txt.clone())
            .render(Rect::new(pos.x, pos.y, txt.len() as u16, 1), buf);

        pos.y += 1;

        let childs = self.childs(from);
        for i in 0..childs.len() {
            let child = childs[i];
            let pref = pref.clone()
                + if is_left { "│ " } else { "  " }
                + if format!("{from}").len() >= 2 {
                    " "
                } else {
                    ""
                };
            self.draw_tree_from(buf, child, pos, i != childs.len() - 1, pref);
        }
    }

    fn number_child_from(&self, from: usize) -> usize {
        let mut res = 1;
        for child in self.childs(from) {
            res += self.number_child_from(child);
        }
        return res;
    }

    fn is_root(&self, child: usize) -> bool {
        return self.parent[child] == None;
    }

    fn two_childs_same_parent(&self) -> Option<(usize, usize)> {
        for i in 1..self.parent.len() {
            let childs = self.childs(i);
            if childs.len() >= 2 {
                return Some((childs[0], childs[1]));
            }
        }
        None
    }

    fn two_root_min_child(&self) -> Option<(usize, usize)> {
        let mut first = None;
        let mut min = NB_LECONS + 10;
        for i in 1..self.parent.len() {
            if !self.is_root(i) {
                continue;
            }
            let nb_child = self.number_child_from(i);
            if nb_child < min {
                min = nb_child;
                first = Some(i);
            }
        }
        match first {
            None => return None,
            Some(first) => {
                min = NB_LECONS + 10;
                let mut second = None;
                for i in 1..self.parent.len() {
                    if !self.is_root(i) || i == first {
                        continue;
                    }
                    let nb_child = self.number_child_from(i);
                    if nb_child < min {
                        min = nb_child;
                        second = Some(i);
                        // todo!();
                    }
                }
                match second {
                    None => return None,
                    Some(second) => return Some((first, second)),
                }
            }
        }
    }
}

impl Ui for Order {
    fn ui(
        &mut self,
        area: Rect,
        buf: &mut Buffer,
        _events: &[Event],
        _mouse: Position,
    ) -> Response {
        let mut pos = Position::new(area.left(), area.top());
        for i in 1..self.parent.len() {
            if self.parent[i] == None {
                self.draw_tree_from(buf, i, &mut pos, false, "".into());
            }
        }

        // self.childs

        Response::NONE
    }
}

const NB_LECONS: usize = 33;
const LECONS_STR: &str = include_str!("../../assets/lecons_agreg.txt");
const LECONS_SHORT_STR : &str = include_str!("../../assets/lecons-short.txt");

#[derive(Debug)]
pub struct Couplage {
    lecons: Vec<&'static str>,
    lecons_short: Vec<&'static str>,
    lecons_choosen: Vec<usize>,

    left: usize,
    right: usize,

    bar_width: u16,

    order: Order,
    ordering_finished: bool,

    stats: Option<[f32; NB_LECONS + 1]>,
}

impl Default for Couplage {
    fn default() -> Self {
        Self {
            lecons: vec![],
            lecons_short: vec![],
            lecons_choosen: vec![],

            left: 0,
            right: 0,

            bar_width: 3,

            order: Order::new(),
            ordering_finished: false,

            stats: None,
        }
    }
}

impl Couplage {
    fn next_left(&mut self) {
        self.lecons_choosen[self.left] += 1;
        self.choose_lecons_smart();
    }
    fn next_right(&mut self) {
        self.lecons_choosen[self.right] += 1;
        self.choose_lecons_smart();
    }

    fn choose_lecons_smart(&mut self) {
        let two_to_merge = self.order.two_childs_same_parent();
        if let Some((first, second)) = two_to_merge {
            self.left = first;
            self.right = second;
            return;
        }
        let roots = self.order.two_root_min_child();
        if let Some((first, second)) = roots {
            self.left = first;
            self.right = second;
            return;
        }

        self.ordering_finished = true;
    }

    fn choose_random_lecons(&mut self) {
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
            self.lecons.push("");
            self.lecons_short.push("");
            self.lecons_choosen.push(0);
            for lecon in parse_lecons(LECONS_STR) {
                self.lecons.push(lecon);
                self.lecons_choosen.push(0);
            }
            for lecon_short in parse_lecons(LECONS_SHORT_STR) {
                self.lecons_short.push(lecon_short);
            }
            self.choose_lecons_smart();
        }
        if should_stop(events) {
            return Response::STOPPED;
        }
        if Zone::default()
            .min_area(Rect::new(
                0,
                0,
                self.lecons.len() as u16 * (self.bar_width + 1) - 1,
                10,
            ))
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
                }) => {
                    self.order.add_relation(self.right, self.left);
                    self.next_right()
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    self.order.add_relation(self.left, self.right);
                    self.next_left();
                }
                _ => {}
            }
        }

        let vert = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Fill(1),
                Constraint::Length(NB_LECONS as u16 + 2 as u16),
                Constraint::Fill(1),
            ])
            .split(area);
        let horizon = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(4),
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
            .text(self.lecons[self.right])
            .ui(right_area, buf, events, mouse)
            .clicked()
        {
            self.order.add_relation(self.right, self.left);
            self.next_right();
        };
        if Zone::default()
            .bordered()
            .text(self.lecons[self.left])
            .ui(left_area, buf, events, mouse)
            .clicked()
        {
            self.order.add_relation(self.left, self.right);
            self.next_left();
        };

        if !self.ordering_finished {
            self.order.ui(vert[2], buf, events, mouse);
            return Response::NONE;
        }

        if self.stats.is_none() {
            let mut vec_stats: [usize; NB_LECONS + 1] = [0; NB_LECONS + 1];

            for i in 1..NB_LECONS + 1 {
                for j in 1..i {
                    match self.order.gt(i, j) {
                        None => {
                            todo!();
                        }
                        Some(true) => vec_stats[i] += 1,
                        Some(false) => vec_stats[j] += 1,
                    }
                }
            }

            let mut stats: [f32; NB_LECONS + 1] = [0 as f32; NB_LECONS + 1];

            for i in 1..NB_LECONS + 1 {
                stats[i] = (vec_stats[i] as f32
                    / (NB_LECONS * (NB_LECONS - 1)) as f32
                    * 2.)
                    * 100.;
            }

            self.stats = Some(stats);
        }

        if let Some(stats) = self.stats {
            let mut txt = vec![];
            for i in 1..NB_LECONS + 1 {
                txt.push(Line::from(format!(
                    "{:<width$} : {:.2} %",
                    self.lecons_short[i],
                    stats[i],
                    width =
                        self.lecons_short.iter().map(|s| s.len()).max().unwrap_or(0)
                )));
            }
            txt.push(Line::from(""));
            let mut total_pourcentage = 0.;
            for i in 1..NB_LECONS + 1 {
                total_pourcentage += stats[i];
            }
            txt.push(Line::from(format!("{}", total_pourcentage)));
            Paragraph::new(txt).render(vert[2], buf);
        }

        /*
                BarChart::new(vec_lecons).bar_width(self.bar_width).render(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints([
                            Constraint::Fill(1),
                            Constraint::Length(
                                self.lecons.len() as u16 * (self.bar_width + 1) - 1,
                            ),
                            Constraint::Fill(1),
                        ])
                        .split(vert[3])[1],
                    buf,
                );
        */
        Response::NONE
    }
}
