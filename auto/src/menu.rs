use std::{error::Error, io};

use cliclack::select;
use colored::ColoredString;

use crate::{cache::ProgressCache, step::{Action, Step}};

pub trait MenuEntry {
    #[allow(unused)]
    fn label(&self, cache: &ProgressCache) -> String;
    fn label_colorized(&self, cache: &ProgressCache) -> ColoredString;
    fn name(&self) -> String;
    fn hint(&self) -> String;
    fn action(&self) -> &dyn Action;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuEntryType {
    Done(DoneAction),
    Step(Step),
}
impl MenuEntryType {
    pub fn list_options() -> Vec<MenuEntryType> {
        let mut my_vec = vec![];

        my_vec.push(MenuEntryType::Done(DoneAction));
        
        Step::list_options().iter().for_each(|entry| {
            my_vec.push(MenuEntryType::Step(*entry));
        });

        my_vec
    }
}
impl MenuEntry for MenuEntryType {
    fn label(&self, cache: &ProgressCache) -> String {
        match self {
            MenuEntryType::Step(step) => step.label(cache),
            MenuEntryType::Done(_) => self.name()
        }
    }

    fn label_colorized(&self, cache: &ProgressCache) -> ColoredString {
        match self {
            MenuEntryType::Step(step) => step.label_colorized(cache),
            MenuEntryType::Done(_) => self.name().into(),
        }
    }

    fn action(&self) -> &dyn Action {
        match self {
            MenuEntryType::Step(step) => step.action(),
            MenuEntryType::Done(action) => action,
        }
    }
    
    fn name(&self) -> String {
        self.action().name()
    }

    fn hint(&self) -> String {
        self.action().hint()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct DoneAction;

impl Action for DoneAction {
    fn invoke(&self) -> Result<Option<crate::step::StepStatus>, Box<dyn Error>> {
        todo!()
    }

    fn name(&self) -> String {
        "Done".into()
    }

    fn hint(&self) -> String {
        "".into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StepConfirmationResponse {
    Confirm,
    Skip,
    Cancel,
}
impl StepConfirmationResponse {
    pub fn prompt(prompt: &str) -> Result<StepConfirmationResponse, io::Error> {
        let response = select(prompt)
            .item(StepConfirmationResponse::Confirm, "Yes", "")
            .item(StepConfirmationResponse::Skip, "Skip", "Mark as skipped")
            .item(StepConfirmationResponse::Cancel, "Cancel", "")
            .interact();

        match response {
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
                Ok(StepConfirmationResponse::Cancel)
            }
            result => result
        }
    }
}
