use std::{collections::HashMap, rc::Rc};

use crate::{interpreter::value::Value, node::FunctionDeclaration};

pub type ScopeId = usize;

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeId>,
    variables: HashMap<String, Rc<Value>>,
    functions: HashMap<String, FunctionDeclaration>,
}

#[derive(Debug)]
pub struct ScopeArena {
    scopes: Vec<Scope>,
}

impl ScopeArena {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    pub fn new_scope(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let scope = Scope {
            parent,
            variables: HashMap::new(),
            functions: HashMap::new(),
        };

        self.scopes.push(scope);
        self.scopes.len() - 1
    }

    pub fn define_variable(&mut self, scope_id: ScopeId, name: impl Into<String>, value: Rc<Value>) {
        self.scopes[scope_id].variables.insert(name.into(), value);
    }

    pub fn define_function(
        &mut self,
        scope_id: ScopeId,
        name: impl Into<String>,
        function: FunctionDeclaration,
    ) {
        self.scopes[scope_id]
            .functions
            .insert(name.into(), function);
    }

    pub fn lookup_variable(&self, mut scope_id: ScopeId, name: &str) -> Option<Rc<Value>> {
        while let Some(scope) = self.scopes.get(scope_id) {
            if let Some(value) = scope.variables.get(name) {
                return Some(value.clone());
            }
            match scope.parent {
                Some(parent) => scope_id = parent,
                None => break,
            }
        }
        None
    }

    pub fn lookup_function(
        &self,
        mut scope_id: ScopeId,
        name: &str,
    ) -> Option<&FunctionDeclaration> {
        while let Some(scope) = self.scopes.get(scope_id) {
            if let Some(function) = scope.functions.get(name) {
                return Some(function);
            }
            match scope.parent {
                Some(parent) => scope_id = parent,
                None => break,
            }
        }
        None
    }
}

impl Default for ScopeArena {
    fn default() -> Self {
        Self::new()
    }
}
