// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
};

pub struct UsageInfoLine {
    pub keys: Vec<String>,
    pub description: String,
}

pub struct UsageInfo {
    pub description: Option<String>,
    pub lines: Vec<UsageInfoLine>,
}

fn key_to_span<'a>(key: &String) -> Span<'a> {
    Span::from(format!("( {} )", key)).bold()
}

pub fn widget_usage_to_text<'a>(usage: UsageInfo) -> Text<'a> {
    let mut lines: Vec<Line> = vec![];

    if let Some(description) = usage.description {
        lines.push(Line::from(description))
    }

    lines.push(Line::from(""));

    for command in usage.lines {
        let mut bindings: Vec<Span> = match command.keys.len() {
            0 => vec![],
            1 => vec![key_to_span(&command.keys[0])],
            2 => vec![
                key_to_span(&command.keys[0]),
                " or ".into(),
                key_to_span(&command.keys[1]),
            ],
            _ => {
                let mut bindings: Vec<Span> = Vec::with_capacity(command.keys.len() * 2);

                for key in command.keys.iter().take(command.keys.len() - 1) {
                    bindings.push(key_to_span(key));
                    bindings.push(", ".into());
                }

                bindings.push("or".into());
                bindings.push(key_to_span(command.keys.last().unwrap()));

                bindings
            }
        };

        bindings.push(Span::from(format!(" {}", command.description)));

        lines.push(Line::from(bindings));
    }

    Text::from(lines)
}
