use crate::{interpreter::{scope::{ScopeArena, ScopeId}, value::Value}, node::FunctionDeclaration};

pub struct ExecutionContext {
    in_function: bool,
    returned_value: Option<Value>,
    scope_arena: ScopeArena,
    current_scope: ScopeId,
}

impl ExecutionContext {
    pub fn new() -> Self {
        let mut scope_arena = ScopeArena::new();
        let current_scope = scope_arena.new_scope(None);

        ExecutionContext {
            in_function: false,
            returned_value: None,
            scope_arena,
            current_scope,
        }
    }

    pub fn enter_function(&mut self) {
        self.in_function = true;
    }

    pub fn exit_function_with_return(&mut self) -> Option<Value> {
        self.in_function = false;
        self.returned_value.take()
    }

    pub fn enter_new_scope(&mut self) -> (usize, usize) {
        let parent_scope = self.current_scope;
        let child_scope = self.scope_arena.new_scope(Some(parent_scope));

        // Enter new scope
        self.current_scope = child_scope;

        (parent_scope, child_scope)
    }

    pub fn define_variable_in_scope(&mut self, identifier: &str, value: Value) {
        self.scope_arena
            .define_variable(
                self.current_scope,
                identifier,
                value
            );
    }

    pub fn define_function_in_scope(&mut self, identifier: &str, node: FunctionDeclaration) {
        self.scope_arena
            .define_function(
                self.current_scope,
                identifier,
                node.clone()
            );
    }

    pub fn lookup_function_in_scope(&mut self, method_name: &str) -> Option<FunctionDeclaration> {
        self.scope_arena
            .lookup_function(
                self.current_scope,
                method_name
            ).cloned()
    }

    pub fn lookup_variable_in_scope(&mut self, identifier: &str) -> Option<&Value> {
        self.scope_arena.lookup_variable(self.current_scope, identifier)
    }

    pub fn restore_scope(&mut self, scope: usize) {
        self.current_scope = scope;
    }

    pub fn is_in_function(&mut self) -> bool {
        self.in_function
    }

    pub fn set_return_value(&mut self, value: Value) {
        self.returned_value = Some(value);
    }
}