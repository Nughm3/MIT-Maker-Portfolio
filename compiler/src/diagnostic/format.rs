use std::{
    collections::VecDeque,
    io::{Result, Write},
};

use yansi::{Color, Paint};

use super::*;
use crate::index::Index;

struct Section<'a>(VecDeque<&'a Label>);

pub fn spacious(index: &Index, report: Report, writer: &mut impl Write) -> Result<()> {
    writeln!(writer)?;

    let location = report.location();
    let source = &index[report.source_id()];
    let lines: Vec<_> = source.contents().lines().collect();

    if yansi::is_enabled() {
        writeln!(
            writer,
            "  {} {}",
            format_args!(" {} ", report.level())
                .bg(report.level().color())
                .black()
                .bold(),
            report.message().bold()
        )?;
    } else {
        writeln!(writer, "  {}: {}", report.level(), report.message())?;
    }

    let max_line_num = report.labels().last().unwrap().location().line();
    let line_num_width = (max_line_num + 1).ilog10() as usize + 1;
    let padding = " ".repeat(line_num_width);
    let bar = "|".blue().bold();
    writeln!(
        writer,
        "  {padding}{} {} {location}",
        "-->".blue().bold(),
        source.path().display().bold()
    )?;

    let mut sections: Vec<Section> = Vec::new();
    for label in report.labels() {
        let label_line = label.location().line();
        if let Some(section) = sections.last_mut() {
            if let Some(last) = section.0.back_mut() {
                if label_line.abs_diff(last.location().line()) <= 3 {
                    section.0.push_back(label);
                    continue;
                }
            }
        }

        sections.push(Section(VecDeque::from([label])));
    }

    for (i, Section(labels)) in sections.iter().enumerate() {
        let start = labels.front().unwrap().location().line().saturating_sub(1);
        let mut end = labels.back().unwrap().location().line() + 1;
        if end < lines.len() - 1 {
            end += 1;
        }

        let mut current_label = 0;
        for (l, line) in lines.iter().enumerate().skip(start).take(end) {
            if let Some(label) = labels.get(current_label) {
                if label.location().line() == l {
                    let color = label.level().color();
                    writeln!(
                        writer,
                        "  {:>line_num_width$} {bar} {line}\n  {padding} {bar} {}{} {}",
                        (l + 1).blue().bold(),
                        " ".repeat(label.location().col()),
                        "^".repeat(label.length()).fg(color).bold(),
                        label.message().fg(color).bold(),
                    )?;
                    current_label += 1;
                    continue;
                }
            }

            writeln!(writer, "  {padding} {bar} {}", lines[l])?;
        }

        if i < sections.len() - 1 {
            writeln!(
                writer,
                "  {:>width$}",
                "...".dim().bold(),
                width = line_num_width + 2
            )?;
        }
    }

    if let Some(note) = report.note {
        writeln!(
            writer,
            "  {padding} {} {} {note}",
            "=".blue().bold(),
            "note:".bold()
        )?;
    }

    Ok(())
}

pub fn compact(source_map: &Index, report: Report, writer: &mut impl Write) -> Result<()> {
    let source = &source_map[report.source_id()];
    let level = report.level();
    let location = report.location();
    writeln!(
        writer,
        "{} ({} {location}) {}",
        level.fg(level.color()).bold(),
        source.path().display().bold(),
        report.message().bold()
    )
}

impl Level {
    fn color(&self) -> Color {
        match self {
            Level::Help => Color::Green,
            Level::Warning => Color::Yellow,
            Level::Error => Color::Red,
        }
    }
}
