#![allow(clippy::cognitive_complexity)]

use std::{fs, io};

use crossterm::event::KeyEventKind;

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let (_, line_size) = terminal::size().unwrap();
    let mut current_id = 0;
    let file_contents = fs::read_to_string("./examples/input.txt")?;
    let mut line_count = 0;
    for _ in file_contents.lines() {
        line_count += 1;
    }
    loop {
        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        w.flush()?;
        let mut current_line = 0;
        for line in file_contents.lines() {
            current_line += 1;
            if current_line < current_id {
                continue;
            }
            if cursor::position()?.1 < line_size - 2 {
                print!("{}", line);
                queue!(w, cursor::MoveToNextLine(1),)?;
            } else {
                break;
            }
        }
        match read_char()? {
            'j' => {
                if current_id < line_count - 1 {
                    current_id += 1
                }
            }
            'k' => {
                if current_id > 1 {
                    current_id -= 1
                }
            }
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }

            _ => {}
        };
        println!("id {}", current_id);
        queue!(w, cursor::MoveToColumn(0),)?;
    }

    execute!(
        w,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}

pub fn read_char() -> std::io::Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            kind: KeyEventKind::Press,
            modifiers: _,
            state: _,
        })) = event::read()
        {
            return Ok(c);
        }
    }
}

pub fn buffer_size() -> io::Result<(u16, u16)> {
    terminal::size()
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();

    run(&mut stdout)
}
