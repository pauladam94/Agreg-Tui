use crossterm::ExecutableCommand;
use crossterm::event::*;
use hello_ratatui::app::App;
use hello_ratatui::event::get;
use hello_ratatui::ui::Ui;
use ratatui::layout::Position;
use std::io;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

fn main() -> io::Result<()> {
    let frame_rate = 60;
    let mut terminal = ratatui::init();
    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;

    let mut app = App::new();
    let mut mouse = Position::new(0, 0);
    let mut stop = false;
    let mut frame_time;

    while !stop {
        frame_time = Instant::now();
        let events = get()?;
        for event in &events {
            if let Event::Mouse(MouseEvent { column, row, .. }) = event {
                mouse.x = *column;
                mouse.y = *row;
            }
        }
        terminal.draw(|frame| {
            if app
                .ui(frame.area(), frame.buffer_mut(), &events, mouse)
                .stopped()
            {
                stop = true;
            }
        })?;

        let elapsed_time = frame_time.elapsed().as_nanos();
        if Duration::new(1, 0).as_nanos() > elapsed_time {
            let rest_frame_time = Duration::new(1, 0).as_nanos() > elapsed_time;
            sleep(Duration::new(0, rest_frame_time as u32));
        }
    }

    std::io::stdout()
        .execute(crossterm::event::DisableMouseCapture)
        .unwrap();
    ratatui::restore();
    Ok(())
}
