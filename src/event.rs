use crossterm::event::{Event, KeyCode, KeyEvent};

pub fn get() -> std::io::Result<Vec<Event>> {
    let mut events = vec![];
    while crossterm::event::poll(std::time::Duration::ZERO)? {
        events.push(crossterm::event::read()?);
    }
    Ok(events)
}

pub fn should_stop(events: &[Event]) -> bool {
    for event in events {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            ..
        }) = event
        {
            return true;
        }
    }
    false
}

pub fn key_pressed(key : char, events : &[Event]) -> bool {
    for event in events {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(key),
            ..
        }) = event
        {
            return true;
        }
    }
    todo!();
    false
}
