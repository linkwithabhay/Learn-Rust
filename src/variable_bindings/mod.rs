use crate::tools::decorator;

pub mod scope;
pub mod shadowing;
pub mod variable_binding;

/// Learn Variable Bindings
pub fn learn() {
    decorator::call("Scope", scope::learn)();

    decorator::call("Shadowing", shadowing::learn)();

    decorator::call("Variable Binding", variable_binding::learn)();
}
