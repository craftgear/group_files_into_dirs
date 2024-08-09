use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::Paragraph,
    Terminal,
};
use std::io::stdout;

use crate::libs::errors::Error;
use crate::libs::extract_keywords::*;

fn handle_keyinputs(key: KeyEvent) -> Result<(), Error> {
    Ok(())
}

fn run_tui(keywords: Vec<(String, usize)>) -> Result<(), Error> {
    println!("keywords is {:?}", keywords);

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

pub fn execute(filenames: Vec<String>) -> Result<(), Error> {
    let keyword_hash = extract_keywords_from_filenames(&filenames);
    let mut keyword_vec: Vec<(String, usize)> = keyword_hash.into_iter().collect();
    keyword_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // filter keywords that appear more than once.
    let keywords = keyword_vec
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect::<Vec<_>>();

    run_tui(keywords)?;

    Ok(())
}
