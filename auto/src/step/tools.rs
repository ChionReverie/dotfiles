use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::{comandline::install_pkglist, menu::StepConfirmationResponse, step::{Action, StepStatus}};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct ToolsStep;

impl Action for ToolsStep {
    fn invoke(&self) -> Result<Option<StepStatus>, Box<dyn Error>> {
        let message = "Install common command-line tools?";
        let response = StepConfirmationResponse::prompt(message)?;

        match response {
            StepConfirmationResponse::Skip => return Ok(Some(StepStatus::skipped_thisversion())),
            StepConfirmationResponse::Cancel => return Ok(None),
            StepConfirmationResponse::Confirm => (),
        }

        install_pkglist("pkg/11-tools.txt")?;

        Ok(Some(StepStatus::done_thisversion()))
    }
    
    fn name(&self) -> String {
        "Common Tools".into()
    }
    
    fn hint(&self) -> String {
        "".into()
    }
}
