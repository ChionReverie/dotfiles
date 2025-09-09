use std::process::Command;

use cliclack::input;
use serde::{Deserialize, Serialize};

use crate::{
    comandline::{ExitStatusError, install_pkglist},
    menu::StepConfirmationResponse,
    step::{Action, StepStatus},
};

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct ZerotierStep;

impl Action for ZerotierStep {
    fn invoke(&self) -> Result<Option<StepStatus>, Box<dyn std::error::Error>> {
        let prompt = "Should this system be connected to a Zerotier network?";
        let response = StepConfirmationResponse::prompt(prompt)?;

        match response {
            StepConfirmationResponse::Skip => return Ok(Some(StepStatus::skipped_thisversion())),
            StepConfirmationResponse::Cancel => return Ok(None),
            StepConfirmationResponse::Confirm => (),
        }

        install_pkglist("pkg/13-zerotier.txt")?;

        Command::new("sudo")
            .arg("systemctl")
            .arg("enable")
            .arg("--now")
            .arg("zerotier-one.service")
            .spawn()?
            .wait()?
            .check_error()?;

        let network_id: String =
            input("Give a Network ID for a Zerotier network to connect to.").interact()?;

        Command::new("sudo")
            .arg("zerotier-cli")
            .arg("join")
            .arg(network_id)
            .spawn()?
            .wait()?
            .check_error()?;

        Ok(Some(StepStatus::done_thisversion()))
    }

    fn name(&self) -> String {
        "Zerotier".into()
    }

    fn hint(&self) -> String {
        "Install Zerotier, and connect to a network".into()
    }
}
