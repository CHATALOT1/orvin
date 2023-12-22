use super::Terminal;
use bevy::prelude::*;
use ratatui::{
    layout::Rect,
    prelude::*,
    style::Style,
    widgets::{block::Title, Block, Borders, Paragraph},
};

pub fn render_system(mut terminal: ResMut<Terminal>) {
    terminal.0.draw(render).expect("Failed to draw terminal");
}

// TODO: Check if frame is too small a frame and complain if so
fn render(frame: &mut Frame, // , input_text: &String
) {
    let frame_size = frame.size();

    // Render Output section
    frame.render_widget(
        Block::new()
            .borders(Borders::ALL)
            .title(Title::from("Output/History").alignment(Alignment::Center)),
        Rect::new(0, 0, frame_size.width, frame_size.height - 4),
    );

    // Render command input section
    let input_block_rect = Rect::new(0, frame_size.height - 4, frame_size.width, 3);
    frame.render_widget(
        Block::new().borders(Borders::ALL).title("Input Command"),
        input_block_rect,
    );

    // Render help text
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            "To escape, use the ".into(),
            "exit".add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
            " command.".into(),
        ]))
        .alignment(Alignment::Left),
        Rect::new(0, frame_size.height - 1, frame_size.width / 2, 1),
    );
    frame.render_widget(
        Paragraph::new(Line::from(vec![
            "For a list of commands and other aid, use the ".into(),
            "help".add_modifier(Modifier::UNDERLINED | Modifier::BOLD),
            " command.".into(),
        ]))
        .alignment(Alignment::Right),
        Rect::new(
            frame_size.width / 2,
            frame_size.height - 1,
            frame_size.width / 2,
            1,
        ),
    );
}
