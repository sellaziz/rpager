#![allow(clippy::cognitive_complexity)]

use std::{fs, io};

use crossterm::{event::KeyEventKind, style::Color};

pub use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, queue, style,
    terminal::{self, ClearType},
    Command,
};

fn limit_line_length<'a, I>(lines: I, max_chars: usize) -> impl Iterator<Item = String> + 'a
where
    I: Iterator<Item = &'a str> + 'a,
{
    lines.flat_map(move |line| {
        let mut remaining = line;

        let mut result = vec![];
        if line.is_empty() {
            result.push("\n".to_string());
        }

        while !remaining.is_empty() {
            let (limited, rest) = remaining.split_at(std::cmp::min(remaining.len(), max_chars));
            result.push(limited.to_string());
            remaining = rest.trim_start();
        }

        result
    })
}

fn print_page<W>(w: &mut W, file_contents: &str, current_id: u32) -> io::Result<()>
where
    W: io::Write,
{
    let (column_size, line_size) = terminal::size().unwrap();
    queue!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    w.flush()?;
    let mut current_line = 0;
    for line in limit_line_length(file_contents.lines(), (column_size - 1) as usize) {
        current_line += 1;
        if current_line < current_id {
            continue;
        }
        if cursor::position()?.1 < line_size - 1 {
            write!(w, "{}", line)?;
            execute!(w, cursor::MoveToNextLine(1),).unwrap();
        }
    }
    Ok(())
}

fn run<W>(w: &mut W) -> io::Result<()>
where
    W: io::Write,
{
    execute!(w, terminal::EnterAlternateScreen)?;

    terminal::enable_raw_mode()?;

    let mut current_id = 0;
    let file_contents = fs::read_to_string("./examples/input.txt")?;
    let mut line_count = 0;
    for _ in file_contents.lines() {
        line_count += 1;
    }

    print_page(w, &file_contents, current_id)?;

    loop {
        match read_char()? {
            'j' => {
                if current_id < line_count - 1 {
                    current_id += 1
                }
                print_page(w, &file_contents, current_id)?;
                execute!(
                    w,
                    style::SetForegroundColor(Color::Black),
                    style::SetBackgroundColor(Color::White),
                    style::Print(":Command "),
                )?;
            }
            'k' => {
                if current_id > 1 {
                    current_id -= 1
                }
                print_page(w, &file_contents, current_id)?;
                execute!(
                    w,
                    style::SetForegroundColor(Color::Black),
                    style::SetBackgroundColor(Color::White),
                    style::Print(":Command "),
                )?;
            }
            'q' => {
                execute!(w, cursor::SetCursorStyle::DefaultUserShape).unwrap();
                break;
            }

            _ => {}
        };
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
