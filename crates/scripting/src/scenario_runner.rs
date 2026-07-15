use core::script_api::ScriptApi;
use std::fmt;

pub struct ScenarioRunner {
}

// #[derive(Debug, thiserror::Error)]
pub enum ScenarioError {
}

impl ScenarioRunner {
    pub fn new(scenario_path: &str) -> Result<Self, ScenarioError> {
        todo!()
    }

    pub fn update(script_api: &mut ScriptApi) {}
}