use super::{
    input::{command::CommandInputState, events::InvalidCommandSubmitted},
    Terminal,
};
use crate::commands::IssueCommand;
use bevy::prelude::*;
use ratatui::{
    layout::Rect,
    prelude::*,
    style::{Color, Style},
    widgets::{block::Title, Block, Borders, Paragraph, Wrap},
};

pub struct CommandFeedback {
    pub timer: Timer,
    pub color: Color,
}
impl Default for CommandFeedback {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.8, TimerMode::Once),
            color: Color::default(),
        }
    }
}

pub fn render_system(
    mut terminal: ResMut<Terminal>,
    command_input_state: Res<CommandInputState>,
    mut command_issued: EventReader<IssueCommand>,
    mut invalid_command_submitted: EventReader<InvalidCommandSubmitted>,
    time: Res<Time>,
    mut command_feedback: Local<CommandFeedback>,
) {
    command_feedback.timer.tick(time.delta());

    // If a valid command was just issued, or an invalid command submitted, determine current
    // input section colour accordingly. Otherwise, reset colour if last command was issued long
    // enough ago.
    if invalid_command_submitted.read().last().is_some() {
        command_feedback.timer.reset();
        command_feedback.color = Color::Red;
    } else if command_issued.read().last().is_some() {
        command_feedback.timer.reset();
        command_feedback.color = Color::Green;
    } else if command_feedback.timer.finished() {
        command_feedback.color = Color::default();
    };

    terminal
        .0
        .draw(|frame| {
            render(
                frame,
                &command_input_state,
                time.elapsed_seconds() % 1. >= 0.5,
                command_feedback.color,
            )
        })
        .expect("Failed to draw terminal");
}

fn render(
    frame: &mut Frame,
    input_state: &CommandInputState,
    cursor_visible: bool,
    input_section_colour: Color,
) {
    let frame_size = frame.size();

    // Especially small
    if frame_size.width < 30 {
        // Best attempt to fit this text into the player's view, so that they can correct it.
        frame.render_widget(
            Paragraph::new("window not large enough")
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false })
                .style(Style::new().red()),
            frame_size,
        );
    }
    // Not especially small, but still too small (text may not display properly)
    else if (frame_size.height < 10) | (frame_size.width < 120) {
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
    // Large enough
    else {
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
            .fg(Color::default())
            .block(Block::new().borders(Borders::ALL).title("Input Command"))
            .fg(input_section_colour),
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
