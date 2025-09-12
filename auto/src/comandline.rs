use std::{
    env,
    error::Error,
    fs::read_to_string,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use regex::Regex;

pub fn relative_to_dotfiles(path: &str) -> PathBuf {
    let home = env::var("DOTFILES_HOME").expect("This should be run using the `auto.sh` script.");
    PathBuf::from(home).join(path)
}

pub fn install_pkglist(path: &str) -> Result<(), Box<dyn Error>> {
    let packages = read_pkglist(relative_to_dotfiles(path))?;

    Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .args(packages)
        .spawn()?
        .wait()?
        .check_error()?;
    Ok(())
}

pub fn read_pkglist(path: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    // TODO: cache
    let pattern =  Regex::new(r"^(?<pkg_name>(?:\w+[ ]*?)*\w+)(?<comment>[ ]*#.*)?$")?;
    
    let packages: Vec<String> = read_to_string(path)?
        .lines()
        .filter_map(|item| {
            let trimmed = item.trim();
            let captures = pattern.captures(trimmed)?;
            let pkg_names = captures.name("pkg_name")?;
            Some(pkg_names.as_str().split_whitespace())
        })
        .flatten()
        .map(String::from)
        .collect();

    Ok(packages)
}

pub trait ExitStatusError {
    fn check_error(&self) -> Result<&Self, Box<dyn Error>>;
}

impl ExitStatusError for ExitStatus {
    fn check_error(&self) -> Result<&Self, Box<dyn Error>> {
        if !self.success() {
            if let Some(code) = self.code() {
                return Err(format!("Exit code {code}").into());
            }
            return Err("Terminated by signal".into());
        }
        Ok(self)
    }
}
