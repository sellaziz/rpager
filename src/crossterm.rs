use crate::app::App;
use crate::ui;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, style,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

pub fn run<W>(
    w: &mut W,
    tick_rate: Duration,
    pattern: String,
    file: String,
) -> Result<(), Box<dyn Error>>
where
    W: io::Write,
{
    // setup terminal
    enable_raw_mode()?;
    execute!(w, EnterAlternateScreen)?;

    // create app and run it
    let app = App::new("Crossterm Demo", pattern, file).unwrap();
    let res = run_app(w, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(w, style::ResetColor, cursor::Show, LeaveAlternateScreen,)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<W>(w: &mut W, mut app: App, tick_rate: Duration) -> io::Result<()>
where
    W: io::Write,
{
    let mut last_tick = Instant::now();
    loop {
        ui::draw(w, &mut app)?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Up => {
                        app.on_up();
                        ui::draw(w, &mut app)?;
                    },
                    KeyCode::Down => {
                        app.on_down();
                        ui::draw(w, &mut app)?;
                    },
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Enter => app.on_enter(),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}
