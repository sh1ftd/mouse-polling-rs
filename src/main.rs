mod app;
mod config;
mod input;
mod ui;

use app::App;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::prelude::*;
use std::{
    error::Error,
    io,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen,)?;

    let terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let app = Arc::new(Mutex::new(App::new()));

    let app_clone = app.clone();
    let input_thread = thread::spawn(move || {
        input::windows::run_input_thread(app_clone);
    });

    let result = run_app(terminal, app);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    if let Err(err) = result {
        println!("{err:?}");
    }

    drop(input_thread);

    Ok(())
}

fn run_app(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
    app: Arc<Mutex<App>>,
) -> io::Result<()> {
    let mut last_update = Instant::now();
    let mut last_activity_check = Instant::now();

    // Force an initial render to properly size the UI at startup
    {
        let app_guard = app.lock().unwrap();
        terminal.draw(|f| ui::ui(f, &app_guard))?;
    }

    loop {
        let terminal_size = size()?;
        let size_ok = terminal_size.0 >= config::MIN_WINDOW_WIDTH
            && terminal_size.1 >= config::MIN_WINDOW_HEIGHT;

        if size_ok {
            if last_update.elapsed() >= config::UPDATE_INTERVAL {
                let mut app_guard = app.lock().unwrap();
                app_guard.check_initialization();

                if app_guard.is_initialized()
                    && last_activity_check.elapsed() >= config::ACTIVITY_CHECK_INTERVAL
                {
                    app_guard.ensure_data_continuity();
                    last_activity_check = Instant::now();
                }

                terminal.draw(|f| ui::ui(f, &app_guard))?;
                drop(app_guard);
                last_update = Instant::now();
            }
        } else {
            terminal.draw(|f| {
                let message = format!(
                    "Terminal too small!\nPlease resize to at least {}x{} characters",
                    config::MIN_WINDOW_WIDTH,
                    config::MIN_WINDOW_HEIGHT
                );
                let paragraph = ratatui::widgets::Paragraph::new(message)
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red));
                f.render_widget(paragraph, f.area());
            })?;
        }

        if event::poll(config::POLLING_INTERVAL)?
            && let Event::Key(key) = event::read()?
                && key.code == KeyCode::Char('q') {
                    return Ok(());
                }

        thread::sleep(config::POLLING_INTERVAL);
    }
}
