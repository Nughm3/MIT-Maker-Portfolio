use std::fmt;

use location::Location;

use crate::index::source::SourceId;

pub mod format;
pub mod location;

// TODO: derive macro
pub trait Diagnostic {
    fn into_report(self) -> Report;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Report {
    level: Level,
    message: String,
    source_id: SourceId,
    location: Location,
    labels: Vec<Label>,
    note: Option<String>,
}

impl Report {
    pub fn new(
        level: Level,
        message: &str,
        source_id: SourceId,
        location: Location,
        length: usize,
    ) -> Self {
        Report {
            level,
            message: message.to_owned(),
            source_id,
            location,
            labels: vec![Label {
                level,
                message: message.to_owned(),
                location,
                length,
            }],
            note: None,
        }
    }

    pub fn with_label(
        mut self,
        level: Level,
        message: &str,
        location: Location,
        length: usize,
    ) -> Self {
        let index = self.labels.partition_point(|l| l.location() < location);
        self.labels.insert(
            index,
            Label {
                level,
                message: message.to_owned(),
                location,
                length,
            },
        );
        self
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.note = Some(note.to_owned());
        self
    }

    pub fn level(&self) -> Level {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source_id(&self) -> SourceId {
        self.source_id
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn labels(&self) -> &[Label] {
        &self.labels
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Label {
    level: Level,
    message: String,
    location: Location,
    length: usize,
}

impl Label {
    pub fn level(&self) -> Level {
        self.level
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Help,
    Warning,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Level::Help => "help",
                Level::Warning => "warning",
                Level::Error => "error",
            }
        )
    }
}
