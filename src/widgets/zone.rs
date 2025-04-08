use crate::ui::Response;
use crate::ui::Ui;
use crossterm::event::*;
use ratatui::layout::*;
use ratatui::prelude::*;
use ratatui::widgets::*;
use unicode_segmentation::UnicodeSegmentation;

use self::symbols::merge::MergeStyle;

#[derive(Default)]
pub struct Zone<'a> {
    dragable: bool,
    drag: Option<&'a mut Offset>,

    border_type: BorderType,
    bordered: bool,

    text: &'a str,

    min_area: Option<Rect>,

    borders: Option<Rect>,

    mouse_follow: bool,

    bg: Option<Color>,

    merge_style: Option<MergeStyle>,
}

fn text_width(s: &str) -> usize {
    s.split('\n')
        .max_by(|x, y| {
            x.graphemes(true).count().cmp(&y.graphemes(true).count())
        })
        .unwrap()
        .graphemes(true)
        .count()
}

#[cfg(test)]
mod test {
    use super::text_width;

    #[test]
    fn essai() {
        assert!(text_width("hey") == 3)
    }

    #[test]
    fn essai2() {
        assert!(
            text_width(
                "hey
hey
heyyyy
"
            ) == 6
        )
    }
}

impl<'a> Zone<'a> {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn dragable(mut self, offset: &'a mut Offset) -> Self {
        self.dragable = true;
        self.drag = Some(offset);
        self
    }
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }
    pub fn merge_style(mut self, merge_style: Option<MergeStyle>) -> Self {
        self.merge_style = merge_style;
        self
    }
    pub fn mouse_followed(mut self) -> Self {
        self.mouse_follow = true;
        self
    }
    pub fn text(mut self, text: &'a str) -> Self {
        self.text = text;
        self
    }
    pub fn bg(mut self, bg: Color) -> Self {
        self.bg = Some(bg);
        self
    }
    pub fn min_area(mut self, min_area: Rect) -> Self {
        self.min_area = Some(min_area);
        self
    }
    pub fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }
}

impl Ui for Zone<'_> {
    fn ui(
        &mut self,
        mut area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        mouse: Position,
    ) -> Response {
        if let Some(min_area) = self.min_area {
            if area.width <= min_area.width || area.height <= min_area.height {
                Text::from(format!(
                    "Should be more than {}x{}",
                    min_area.width, min_area.height
                ))
                .render(Rect::new(10, 10, 100, 1), buf);
                return Response::STOPPED;
            }
        }

        if let Some(bg) = self.bg {
            for x in area.left()..area.right() {
                for y in area.top()..area.bottom() {
                    buf[(x, y)].set_bg(bg);
                }
            }
        }

        let mut rep = Response::default();
        let offset = match &self.drag {
            None => &Offset::default(),
            Some(offset) => offset,
        };

        let offset = *offset;
        if area.offset(offset).contains(mouse) {
            rep = rep.hover()
        }
        for event in events {
            match event {
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    ..
                }) => {
                    if rep.hovered() {
                        rep = rep.click();
                    }
                }
                Event::Mouse(MouseEvent {
                    kind: MouseEventKind::Drag(MouseButton::Left),
                    column,
                    row,
                    ..
                }) => {
                    if rep.hovered() {
                        rep = rep.click();
                        if let Some(offset_mut) = &mut self.drag {
                            **offset_mut = Offset::new(
                                *column as i32 - area.left() as i32 + offset.x,
                                *row as i32 - area.top() as i32 + offset.y,
                            );
                        }
                    }
                }
                _ => {}
            }
        }

        if self.mouse_follow {
            area = Rect::new(mouse.x, mouse.y, area.width, area.height);
        }

        let block = if self.bordered {
            Block::bordered()
        } else {
            Block::default()
        };

        Paragraph::new(self.text)
            .centered()
            .wrap(Wrap { trim: true })
            .block(
                block
                    .merge_style(self.merge_style.clone())
                    .border_style(if rep.clicked() {
                        Style::new().green()
                    } else if rep.hovered() {
                        Style::new().fg(Color::Rgb(255, 128, 0))
                    } else {
                        Style::new()
                    })
                    .border_type(self.border_type),
            )
            .render(
                Rect::new(
                    (area.x as i32 + offset.x) as u16,
                    (area.y as i32 + offset.y) as u16,
                    area.width,
                    area.height,
                ),
                buf,
            );
        /*
                if area.left() + area.right() < text_width(self.text) as u16
                    || area.right() - area.left() < text_width(self.text) as u16 + 2
                {
                    let mut line = (area.top() + area.bottom()) / 2;
                    let mut col = area.left() + 1;
                    for word in self.text.split(" ") {
                        if col + word.len() as u16 + 1 > area.right() {
                            col = area.left() + 1;
                            line += 1;
                        }
                        Text::from(word)
                            .render(Rect::new(col, line, word.len() as u16, 1), buf);
                        col += word.len() as u16 + 1;
                    }
                } else {
                    let text_x =
                        (area.left() + area.right() - text_width(self.text) as u16) / 2;
                    Text::from(self.text).render(
                        Rect::new(
                            text_x,
                            (area.top() + area.bottom()) / 2,
                            area.width,
                            area.height,
                        ),
                        buf,
                    );
                };
        */

        rep
    }
}
