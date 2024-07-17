use super::{Component, ComponentRender};
use crate::state_store::{Action, State};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Rect,
    style::{Color, Style, Styled, Stylize},
    widgets::{Block, Borders, Paragraph},
};
use tokio::sync::mpsc::UnboundedSender;

pub struct output_box<T>{
    content: Vec<T>,
}

impl<T> output_box<T>{
    pub fn content(&self) -> &Vec<T>{
        &self.content
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn clear(&mut self) {
        self.content.clear();
    }

    

}

pub struct RenderProperties {
    pub title: String,
    pub area: Rect,
    pub border_color: Color,
    pub show_cursor: bool,
}

impl ComponentRender<RenderProperties> for output_box {
    fn render(&self, frame: &mut ratatui::prelude::Frame, properties: RenderProperties) {
        let output = Paragraph::new(for n in 0..content.len({

        }))
            .style(Style::default().fg(Color::Cyan))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .fg(properties.border_color)
                    .title(properties.title),
            );
        frame.render_widget(input, properties.area);

        if properties.show_cursor {
            frame.set_cursor(
                properties.area.x + self.cursor_position as u16 + 1,
                properties.area.y + 1,
            )
        }
    }
}

