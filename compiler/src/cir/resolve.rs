use ahash::{HashMap, HashMapExt};
use cstree::interning::TokenKey;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Env<T> {
    scopes: Vec<HashMap<TokenKey, T>>,
    parents: Vec<usize>,
    restricted: Vec<usize>,
}

impl<T> Default for Env<T> {
    fn default() -> Self {
        Env {
            scopes: Vec::new(),
            parents: Vec::new(),
            restricted: Vec::new(),
        }
    }
}

impl<T> Env<T> {
    pub fn new() -> Self {
        Env::default()
    }

    pub fn scope(&mut self) {
        self.parents.push(self.scopes.len());
        self.scopes.push(HashMap::new());
    }

    pub fn restricted_scope(&mut self) {
        self.restricted.push(self.scopes.len());
        self.scope();
    }

    pub fn subscope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn unscope(&mut self) {
        let len = self.parents.pop().unwrap_or_default();
        self.scopes.truncate(len);
    }

    pub fn bind(&mut self, name: TokenKey, value: T) -> Result<(), T> {
        let scope = self.scopes.last_mut().expect("not in any scope");
        match scope.insert(name, value) {
            Some(old) => Err(old),
            None => Ok(()),
        }
    }

    pub fn resolve(&mut self, name: TokenKey) -> Option<&T> {
        for (i, scope) in self.scopes.iter().enumerate().rev() {
            if let Some(value) = scope.get(&name) {
                return Some(value);
            }

            if self.restricted.last() == Some(&i) {
                break;
            }
        }

        None
    }
}
