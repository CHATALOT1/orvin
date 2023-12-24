use super::{input::CommandInputState, Terminal};
use bevy::{
    ecs::system::{Res, ResMut},
    time::Time,
};
use ratatui::{
    layout::Rect,
    prelude::*,
    style::Color,
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
};

pub fn render_system(
    mut terminal: ResMut<Terminal>,
    command_input_state: Res<CommandInputState>,
    time: Res<Time>,
) {
    terminal
        .0
        .draw(|frame| {
            render(
                frame,
                &command_input_state,
                time.elapsed_seconds() % 1. >= 0.5,
            )
        })
        .expect("Failed to draw terminal");
}

fn render(frame: &mut Frame, input_state: &CommandInputState, cursor_visible: bool) {
    let frame_size = frame.size();

    // Display error if window is too small
    // (Width requirement is the lowest resolution that all the help text still displays at)
    if (frame_size.height < 10) | (frame_size.width < 120) {
        if frame_size.width < 30 {
            frame.render_widget(
                Paragraph::new("window not large enough")
                    .alignment(Alignment::Center)
                    .wrap(Wrap { trim: false })
                    .style(Style::new().red()),
                frame_size,
            );
        } else {
            frame.render_widget(
                Paragraph::new(vec![
                    "".into(),
                    "window must be atleast 120x10".into(),
                    "".into(),
                    format!(
                        "current size is {0} x {1}",
                        frame_size.width, frame_size.height
                    )
                    .into(),
                ])
                .alignment(Alignment::Center)
                .style(Style::new().red())
                .wrap(Wrap { trim: false })
                .block(
                    Block::new()
                        .borders(Borders::ALL)
                        .title(Title::from("window not large enough").alignment(Alignment::Center)),
                ),
                frame_size,
            );
        }
    } else {
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
            Paragraph::new(if cursor_visible {
                if input_state.cursor_pos == input_state.content.chars().count() {
                    Line::from(vec![input_state.content.clone().into(), "_".into()])
                } else {
                    Line::from(vec![
                        input_state.content[..input_state.cursor_pos].into(),
                        input_state
                            .content
                            .chars()
                            .nth(input_state.cursor_pos)
                            .unwrap()
                            .to_string()
                            .bg(Color::White)
                            .fg(Color::Black),
                        input_state.content[input_state.cursor_pos + 1..].into(),
                    ])
                }
            } else {
                Line::from(input_state.content.clone())
            })
            .block(Block::new().borders(Borders::ALL).title("Input Command")),
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
}
