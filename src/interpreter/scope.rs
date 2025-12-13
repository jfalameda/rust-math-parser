use std::collections::HashMap;

use crate::interpreter::value::Value;


pub type ScopeId = usize;

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    variables: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct ScopeArena {
    scopes: Vec<Scope>,
}

impl ScopeArena {
    /// Create a new empty arena
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    /// Create a new scope
    pub fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let scope = Scope {
            parent,
            variables: HashMap::new(),
        };

        self.scopes.push(scope);
        self.scopes.len() - 1
    }

    /// Define a variable in the current scope
    pub fn define(
        &mut self,
        scope_id: ScopeId,
        name: impl Into<String>,
        value: Value,
    ) {
        self.scopes[scope_id]
            .variables
            .insert(name.into(), value);
    }

    pub fn lookup(&self, mut scope_id: ScopeId, name: &str) -> Option<&Value> {
        while let Some(scope) = self.scopes.get(scope_id) {
            if let Some(value) = scope.variables.get(name) {
                return Some(value);
            }
            match scope.parent {
                Some(parent) => scope_id = parent,
                None => break,
            }
        }
    None
}
}