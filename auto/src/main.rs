use std::{env, error::Error, sync::LazyLock};

use cliclack::{intro, log, outro, select};
use serde_semver::semver::Version;

use crate::{
    cache::ProgressCache, menu::{MenuEntry, MenuEntryType}, step::{Step, StepStatus, firstboot::FirstBoot}
};

mod step;
mod menu;
mod comandline;
mod cache;

pub static BUILD_VERSION: LazyLock<Version> =
    LazyLock::new(|| Version::parse(env!("CARGO_PKG_VERSION")).expect("Must be a valid semver"));

fn main() -> Result<(), Box<dyn Error>> {
    // We depend on version for future steps.
    let _ = BUILD_VERSION.to_owned();
    let _ = env::var("DOTFILES_HOME").expect("This should be run using the `auto.sh` script.");

    let mut cache = ProgressCache::load()?;
    
    intro(format!("Chion's Dotfiles tool {}", *BUILD_VERSION))?;

    // First Boot
    let step = Step::FirstBoot(FirstBoot);
    let firstboot_entry = cache.entry(step);
    if let StepStatus::Todo = firstboot_entry.status {
        let status = match step.action().invoke()? {
            None => {
                return Ok(());
            }
            Some(status) => status,
        };

        // Update status
        let entry = cache.entry(step).with_status(status);
        cache.insert(step, entry.clone());
        cache.save()?;

        if let StepStatus::Todo = entry.status {
            return Ok(());
        }
    }

    // Steps forward
    while let Some(option) = prompt_stepmenu(&cache, "Which step to begin?") {
        let step = match option {
            MenuEntryType::Done(_) => break,
            MenuEntryType::Step(step) => step,
        };
        
        let status = match step.action().invoke() {
            Ok(status) => status,
            Err(error) =>  {
                log::error(format!("Something went wrong: {}", error))?;
                log::info("Returning to step selection.")?;
                Some(StepStatus::error_thisversion()) 
            },
        };
        let status = match status {
            Some(status) => status,
            None => continue,
        };
        let entry = cache.entry(step).with_status(status);
        cache.insert(step, entry);
        cache.save()?;
    }

    // Exit
    outro("Done!")?;
    Ok(())
}

pub fn prompt_stepmenu(
    cache: &ProgressCache,
    prompt: &str,
) -> Option<MenuEntryType> {
    let mut select = select::<MenuEntryType>(prompt);

    let options = MenuEntryType::list_options();

    for option in options {
        select = select.item(option, option.label_colorized(cache), option.hint());
    }

    select.interact().ok()
}
