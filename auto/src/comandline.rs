use std::{
    env,
    error::Error,
    fs::read_to_string,
    path::PathBuf,
    process::{Command, ExitStatus},
};

pub fn dotfiles_relative(path: &str) -> PathBuf {
    let home = env::var("DOTFILES_HOME").expect("This should be run using the `auto.sh` script.");
    PathBuf::from(home).join(path)
}

pub fn install_pkglist(path: &str) -> Result<(), Box<dyn Error>> {
    let pkglist = dotfiles_relative(path);
    let packages = read_to_string(pkglist)?
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    Command::new("sudo")
        .arg("pacman")
        .arg("-Syu")
        .args(packages)
        .spawn()?
        .wait()?
        .check_error()?;
    Ok(())
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
