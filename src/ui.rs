use crate::app::App;
use crossterm::{
    cursor, execute, queue, style,
    style::Color,
    terminal::{self, ClearType},
};
use std::io;

pub fn render_cmdline<W>(w: &mut W, app: &mut App) -> io::Result<()>
where
    W: io::Write,
{
    let (column_size, line_size) = terminal::size().unwrap();
    let cmd_str = app.cmd_str.clone();
    let status:String;
    if let Some(idx) = app.finder.idx {
        status = format!("search:({}/{}),({},{}):{:3}%",
            idx, app.finder.found_items.len(),
            app.current_id, app.line_count, ((app.current_id as f32/app.line_count as f32)*100.0) as u16);

    } else {
        status = format!("({},{}):{:3}%",app.current_id, app.line_count, ((app.current_id as f32/app.line_count as f32)*100.0) as u16);
    }

    execute!(
        w,
        style::SetForegroundColor(Color::Black),
        style::SetBackgroundColor(Color::White),
        cursor::MoveTo(0, line_size),
        style::Print(cmd_str),
        cursor::MoveTo(column_size - status.len() as u16, line_size),
        style::Print(status),
        style::ResetColor,
    )?;

    Ok(())
}

pub fn draw<W>(w: &mut W, app: &mut App) -> io::Result<()>
where
    W: io::Write,
{
    let (column_size, line_size) = terminal::size().unwrap();
    execute!(
        w,
        style::ResetColor,
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    w.flush()?;
    let mut current_line = 0;
    for line in &app.adjusted_file_contents {
        current_line += 1;
        if current_line < app.current_id {
            continue;
        }
        if cursor::position()?.1 < line_size - 1 {
            execute!(w, style::ResetColor,)?;
            if let Some(cur_word) = app.finder.word.clone() {
                let word = cur_word.clone();
                if let Some(index) = line.to_lowercase().find(&word[..]) {
                    for (i, c) in line.char_indices() {
                        if i >= index {
                            if i >= index + word.len() {
                                execute!(w, style::ResetColor,)?;
                            } else {
                                execute!(
                                    w,
                                    style::SetForegroundColor(Color::Black),
                                    style::SetBackgroundColor(Color::White),
                                )?;
                            }
                        }
                        write!(w, "{}", c)?;
                    }
                } else {
                    write!(w, "{}", line)?;
                }
            } else {
                write!(w, "{}", line)?;
            }
            execute!(w, cursor::MoveToNextLine(1),).unwrap();
        }
    }
    let _ = render_cmdline(w, app);
    Ok(())
}

