use std::{
    hash::{DefaultHasher, Hash, Hasher},
    io::{Error, ErrorKind, Read, Result},
    ops,
    path::{Path, PathBuf},
    time::SystemTime,
};

use cstree::build::NodeCache;

use super::Index;
use crate::{
    parser::{parse, ParseError},
    syntax::{ast::File, AstElement, SyntaxNode},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct SourceId(u32);
cranelift_entity::entity_impl!(SourceId);

#[derive(Debug)]
pub struct Source {
    path: PathBuf,
    contents: String,
    line_breaks: Vec<usize>,
    mtime: Option<SystemTime>,
    hash: u64,
    cst: SyntaxNode,
    parse_errors: Vec<ParseError>,
}

impl Source {
    pub fn load(
        root: impl AsRef<Path>,
        path: impl AsRef<Path>,
        node_cache: &mut NodeCache,
    ) -> Result<Self> {
        let path = path.as_ref().strip_prefix(&root).map_err(|e| {
            Error::new(
                ErrorKind::NotFound,
                format!("cannot load path outside of source tree: {e}"),
            )
        })?;

        let mut file = std::fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mtime = file
            .metadata()
            .and_then(|metadata| metadata.modified())
            .ok();

        let hash = {
            let mut state = DefaultHasher::new();
            contents.hash(&mut state);
            state.finish()
        };

        let line_breaks = contents
            .char_indices()
            .filter_map(|(i, c)| (c == '\n').then_some(i))
            .collect();

        let (cst, parse_errors) = parse(&contents, node_cache);

        Ok(Source {
            path: path.to_owned(),
            contents,
            line_breaks,
            mtime,
            hash,
            cst,
            parse_errors,
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn line_breaks(&self) -> &[usize] {
        &self.line_breaks
    }

    pub fn mtime(&self) -> Option<SystemTime> {
        self.mtime
    }

    pub fn hash(&self) -> u64 {
        self.hash
    }

    pub fn cst(&self) -> &SyntaxNode {
        &self.cst
    }

    pub fn ast(&self) -> Option<File> {
        File::cast(self.cst.clone().into())
    }

    pub fn parse_errors(&self) -> &[ParseError] {
        &self.parse_errors
    }
}

impl ops::Index<SourceId> for Index {
    type Output = Source;

    fn index(&self, source_id: SourceId) -> &Self::Output {
        &self.sources[source_id]
    }
}
