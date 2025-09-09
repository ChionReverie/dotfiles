use std::{error::Error, io};

use cliclack::{confirm, note, outro_cancel};
use indoc::indoc;
use serde::{Deserialize, Serialize};

use crate::{step::{Action, StepStatus}};

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct FirstBoot;

impl Action for FirstBoot {
    fn invoke(&self) -> Result<Option<StepStatus>, Box<dyn Error>> {
        prompt()
    }
    
    fn name(&self) -> String {
        "First Boot".into()
    }

    fn hint(&self) -> String {
        "Repeat first boot prompt".into()
    }
}

fn prompt() -> Result<Option<StepStatus>, Box<dyn Error>> {
    note(
        "First Boot",
        indoc! {"
            This tool assumes you have already installed Arch Linux with the following:
            - Already-configured disk partitions
            - Arch `base` and `linux` packages installed
            - Basic networking (a connection to the internet)
            - Grub Bootloader
            - A root user with a defined password
            - A set hostname for the system
            - Time and Localization information selected
        "},
    )?;

    let reponse = confirm("Have you completed all the above steps?")
        .initial_value(false)
        .interact();
    let is_user_ready = match reponse {
        Ok(boolean) => boolean,
        Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {
            return Ok(None);
        }
        Err(e) => return Err(e.into()),
    };

    if !is_user_ready {
        outro_cancel("Please complete the above steps before proceeding.")?;
        return Ok(Some(StepStatus::Todo));
    }

    Ok(Some(StepStatus::done_thisversion()))
}
