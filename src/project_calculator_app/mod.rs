use crate::tools::decorator;

pub mod calculator_app;

// Learn through Projects
pub fn learn() {
    decorator::call("Project: Calculator App", calculator_app::learn)();
}
