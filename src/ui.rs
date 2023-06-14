use crate::app::App;
use crossterm::{
    cursor, execute, queue, style,
    style::Color,
    terminal::{self, ClearType},
};
use std::io;
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

pub fn draw<W>(w: &mut W, app: &mut App) -> io::Result<()>
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
    for line in limit_line_length(app.file_contents.lines(), (column_size - 1) as usize) {
        current_line += 1;
        if current_line < app.current_id {
            continue;
        }
        if cursor::position()?.1 < line_size - 1 {
            write!(w, "{}", line)?;
            execute!(w, cursor::MoveToNextLine(1),).unwrap();
        }
    }
    execute!(
        w,
        style::SetForegroundColor(Color::Black),
        style::SetBackgroundColor(Color::White),
        style::Print(":Command "),
    )?;
    Ok(())
}

