use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_semver::semver::Version;

use colored::{ColoredString, Colorize};

use crate::{
    BUILD_VERSION,
    cache::ProgressCache,
    menu::MenuEntry,
    step::{firstboot::FirstBoot, networking::ZerotierStep, tools::ToolsStep},
};

pub mod firstboot;
pub mod networking;
pub mod tools;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub enum Step {
    FirstBoot(FirstBoot),
    Tools(ToolsStep),
    Zerotier(ZerotierStep),
}
impl Step {
    pub fn list_options() -> Vec<Self> {
        vec![
            Step::FirstBoot(FirstBoot),
            Step::Tools(ToolsStep),
            Step::Zerotier(ZerotierStep),
        ]
    }
}

pub trait Action {
    fn invoke(&self) -> Result<Option<StepStatus>, Box<dyn Error>>;
    fn name(&self) -> String;
    fn hint(&self) -> String;
}

impl MenuEntry for Step {
    fn label(&self, cache: &ProgressCache) -> String {
        let cache_entry = cache.entry(*self);
        let my_str = format!("{} - {}", self.name(), cache_entry.status);
        my_str
    }

    fn label_colorized(&self, cache: &ProgressCache) -> ColoredString {
        let cache_entry = cache.entry(*self);
        format!("{} - {}", self.name(), cache_entry.status.colorize()).into()
    }

    fn name(&self) -> String {
        self.action().name()
    }

    fn hint(&self) -> String {
        self.action().hint()
    }

    fn action(&self) -> &dyn Action {
        match self {
            Step::FirstBoot(action) => action,
            Step::Tools(action) => action,
            Step::Zerotier(action) => action,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub enum StepStatus {
    #[default]
    Todo,
    Skipped(Version),
    Done(Version),
    Error(Version),
}
impl StepStatus {
    pub fn done_thisversion() -> Self {
        StepStatus::Done(BUILD_VERSION.clone())
    }

    pub fn skipped_thisversion() -> StepStatus {
        StepStatus::Skipped(BUILD_VERSION.clone())
    }

    pub fn error_thisversion() -> StepStatus {
        StepStatus::Error(BUILD_VERSION.clone())
    }

    pub fn colorize(&self) -> ColoredString {
        match self {
            StepStatus::Todo => self.to_string().green(),
            StepStatus::Skipped(_) => self.to_string().yellow(),
            StepStatus::Done(_) => self.to_string().blue(),
            StepStatus::Error(_) => self.to_string().red(),
        }
    }
}
impl Display for StepStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StepStatus::Todo => write!(f, "Todo"),
            StepStatus::Skipped(version) => write! {f, "Skipped ({})", version},
            StepStatus::Done(version) => write!(f, "Done ({})", version),
            StepStatus::Error(version) => write!(f, "Error ({})", version),
        }
    }
}
