use std::{
    cell::OnceCell,
    fs,
    io::{Error, ErrorKind, Result},
    ops,
    path::{Path, PathBuf},
};

use ahash::HashMap;
use cranelift_entity::{EntitySet, PrimaryMap};
use cstree::{
    build::NodeCache,
    interning::{Interner, Resolver, TokenInterner, TokenKey},
};
use source::{Source, SourceId};

use crate::syntax::{
    ast::{Constant, Function, Item, Path as AstPath, TypeDef},
    AstInternedToken,
};

pub mod source;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(u32);

#[derive(Debug)]
pub struct Index {
    root: PathBuf,
    version: Version,
    node_cache: NodeCache<'static>,
    sources: PrimaryMap<SourceId, Source>,
    globals: PrimaryMap<GlobalId, Global>,
    contents: OnceCell<Module>,
}

impl Index {
    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn version(&self) -> Version {
        self.version
    }

    pub fn interner(&self) -> &TokenInterner {
        self.node_cache.interner()
    }

    pub fn interner_mut(&mut self) -> &mut TokenInterner {
        self.node_cache.interner_mut()
    }

    pub fn contents(&self) -> &Module {
        self.contents.get().expect("index not initialized")
    }

    pub fn load(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();
        let mut index = Index {
            root: root.to_path_buf(),
            version: Version::default(),
            node_cache: NodeCache::new(),
            sources: PrimaryMap::new(),
            globals: PrimaryMap::new(),
            contents: OnceCell::new(),
        };
        let mut module = Module::new();
        index.load_rec(root, &mut module)?;
        index.contents.set(module).unwrap();
        Ok(index)
    }

    fn load_rec(&mut self, dir: &Path, module: &mut Module) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            let file_stem = path
                .file_stem()
                .expect("no file stem")
                .to_str()
                .expect("filename must be UTF-8");
            let key = self.interner_mut().get_or_intern(file_stem);

            if path.is_dir() {
                let nested = module.children.entry(key).or_default();
                self.load_rec(&path, nested)?;
            } else if let Some("nw") = path.extension().and_then(|ext| ext.to_str()) {
                if file_stem == "package" {
                    return Err(Error::new(
                        ErrorKind::Other, // TODO: ErrorKind::InvalidFilename
                        "file name `package.nw` is reserved",
                    ));
                } else {
                    let source_id = Source::load(&self.root, &path, &mut self.node_cache)
                        .map(|source| self.sources.push(source))?;
                    if file_stem == "main" {
                        self.load_module(source_id, module)?;
                    } else {
                        let mut nested = Module::new();
                        self.load_module(source_id, &mut nested)?;
                        module.children.insert(key, nested);
                    }
                }
            }
        }

        Ok(())
    }

    fn load_module(&mut self, source_id: SourceId, module: &mut Module) -> Result<()> {
        module.source_id.set(source_id).unwrap();

        if let Some(ast) = self[source_id].ast() {
            for item in ast.items() {
                match item {
                    Item::Import(import) => module.imports.push(import.path().unwrap()),
                    Item::TypeDef(type_def) => {
                        module.globals.insert(self.globals.push(Global {
                            name: type_def.name().unwrap().text_key(),
                            kind: GlobalKind::TypeDef(type_def),
                            updated: Version::default(),
                        }));
                    }
                    Item::Function(function) => {
                        module.globals.insert(self.globals.push(Global {
                            name: function.signature().unwrap().name().unwrap().text_key(),
                            kind: GlobalKind::Function(function),
                            updated: Version::default(),
                        }));
                    }
                    Item::Constant(constant) => {
                        module.globals.insert(self.globals.push(Global {
                            name: constant.name().unwrap().text_key(),
                            kind: GlobalKind::Constant(constant),
                            updated: Version::default(),
                        }));
                    }
                }
            }
        }

        Ok(())
    }

    pub fn resolve_ast_path<'a>(&'a self, path: AstPath, module: &'a Module) -> Resolved<'a> {
        let components: Vec<_> = path
            .path_components()
            .map(|c| c.name().unwrap().text_key())
            .collect();

        let module = if path.absolute().is_some() {
            self.contents()
        } else {
            module
        };

        self.resolve_path(&components, module)
    }

    pub fn resolve_path<'a>(&'a self, components: &[TokenKey], module: &'a Module) -> Resolved<'a> {
        match components.split_first() {
            Some((&head, tail)) => {
                if let Some(nested) = module.children.get(&head) {
                    self.resolve_path(tail, nested)
                } else {
                    Resolved {
                        module,
                        global_id: module
                            .globals()
                            .find(|&global_id| self[global_id].name() == head),
                        remainder: tail.to_vec(),
                    }
                }
            }
            None => Resolved {
                module,
                global_id: None,
                remainder: components.to_vec(),
            },
        }
    }
}

impl ops::Index<TokenKey> for Index {
    type Output = str;

    fn index(&self, key: TokenKey) -> &Self::Output {
        self.interner().resolve(key)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Module {
    source_id: OnceCell<SourceId>,
    updated: Version,
    imports: Vec<AstPath>,
    globals: EntitySet<GlobalId>,
    children: HashMap<TokenKey, Module>,
    used: bool,
}

impl Module {
    pub fn new() -> Self {
        Module::default()
    }

    pub fn source_id(&self) -> SourceId {
        *self.source_id.get().expect("source ID not initialized")
    }

    pub fn updated(&self) -> Version {
        self.updated
    }

    pub fn imports(&self) -> &[AstPath] {
        &self.imports
    }

    pub fn globals(&self) -> impl Iterator<Item = GlobalId> {
        self.globals.keys()
    }

    pub fn used(&self) -> bool {
        self.used
    }

    pub fn mark_used(&mut self) {
        self.used = true;
    }
}

impl ops::Index<TokenKey> for Module {
    type Output = Self;

    fn index(&self, key: TokenKey) -> &Self::Output {
        &self.children[&key]
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct GlobalId(u32);
cranelift_entity::entity_impl!(GlobalId);

#[derive(Debug, Clone)]
pub struct Global {
    name: TokenKey,
    kind: GlobalKind,
    updated: Version,
}

impl Global {
    pub fn name(&self) -> TokenKey {
        self.name
    }

    pub fn kind(&self) -> &GlobalKind {
        &self.kind
    }

    pub fn updated(&self) -> Version {
        self.updated
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum GlobalKind {
    TypeDef(TypeDef),
    Function(Function),
    Constant(Constant),
}

impl ops::Index<GlobalId> for Index {
    type Output = Global;

    fn index(&self, id: GlobalId) -> &Self::Output {
        &self.globals[id]
    }
}

#[derive(Debug, Clone)]
pub struct Resolved<'a> {
    module: &'a Module,
    global_id: Option<GlobalId>,
    remainder: Vec<TokenKey>,
}

impl Resolved<'_> {
    pub fn module(&self) -> &Module {
        self.module
    }

    pub fn global_id(&self) -> Option<GlobalId> {
        self.global_id
    }

    pub fn remainder(&self) -> &[TokenKey] {
        &self.remainder
    }
}
