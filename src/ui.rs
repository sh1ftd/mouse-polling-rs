use ratatui::{
    prelude::*,
    symbols,
    widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph},
};

use crate::{app::App, config};

pub fn ui(f: &mut Frame, app: &App) {
    let terminal_area = f.area();

    let app_area = Rect::new(
        // Center horizontally
        terminal_area
            .x
            .saturating_add(terminal_area.width.saturating_sub(config::MAX_WINDOW_WIDTH) / 2),
        // Center vertically
        terminal_area.y.saturating_add(
            terminal_area
                .height
                .saturating_sub(config::MAX_WINDOW_HEIGHT)
                / 2,
        ),
        terminal_area.width.min(config::MAX_WINDOW_WIDTH),
        terminal_area.height.min(config::MAX_WINDOW_HEIGHT),
    );

    let background = Block::default()
        .style(Style::default().bg(Color::Black))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));
    f.render_widget(background, app_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(7),
        ])
        .split(app_area);

    render_title(f, chunks[0]);
    render_chart(f, app, chunks[1]);
    render_stats(f, app, chunks[2]);
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("Mouse Polling Rate Monitor (Press 'q' to quit)")
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);
    f.render_widget(title, area);
}

fn render_chart(f: &mut Frame, app: &App, area: Rect) {
    if !app.is_initialized() {
        let init_message = Paragraph::new("Initializing...")
            .block(Block::default().title("Polling Rate").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(init_message, area);
        return;
    }

    let data_points: Vec<_> = app.rate_history.iter().map(|&(x, y)| (x, y)).collect();

    let dataset = Dataset::default()
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data_points);

    let now = app.start_time.elapsed().as_secs_f64();
    let x_max = now;
    let x_min = x_max - config::MIN_TIME_WINDOW;

    let chart = Chart::new(vec![dataset])
        .block(Block::default().title("Polling Rate").borders(Borders::ALL))
        .x_axis(Axis::default().bounds([x_min, x_max]).labels(vec![
            Span::raw(format!("{:.1}s", x_min)),
            Span::raw(format!("{:.1}s", x_max)),
        ]))
        .y_axis(
            Axis::default()
                .bounds([0.0, app.graph_max_rate])
                .labels(vec![
                    Span::raw("0"),
                    Span::raw(format!("{:.0}", app.graph_max_rate / 2.0)),
                    Span::raw(format!("{:.0}", app.graph_max_rate)),
                ]),
        );

    f.render_widget(chart, area);
}

fn render_stats(f: &mut Frame, app: &App, area: Rect) {
    let current_rate = app.calculate_current_rate();
    let avg_rate = app.calculate_avg_rate(5.0);

    let stats = if !app.is_initialized() {
        "Initializing...".to_string()
    } else {
        format!(
            "Current Rate: {:.0} Hz\n\
             Average Rate (5s): {:.0} Hz\n\
             Maximum Rate: {:.0} Hz\n\
             Mouse Position: {:?}\n\
             Time Window: {:.1}s",
            current_rate,
            avg_rate,
            app.max_rate,
            app.current_pos,
            config::MIN_TIME_WINDOW
        )
    };

    let stats_widget =
        Paragraph::new(stats).block(Block::default().title("Statistics").borders(Borders::ALL));

    f.render_widget(stats_widget, area);
}
//
