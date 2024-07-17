use std::process::Command;
use dyn_clone::DynClone;

#[derive(Clone)]
pub struct RunCommand {}

pub trait RunCommandTrait: DynClone + Send {
    fn run_command(&mut self, command: String) -> String;
}

impl RunCommandTrait for RunCommand {
    fn run_command(&mut self, command: String) -> String {
        run_command(command)
    }
}

pub fn run_command(command: String) -> String {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command.as_str()])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("/bin/bash")
            .arg("-c")
            .arg(command.as_str())
            .output()
            .expect("failed to execute process")
    };
    String::from_utf8(output.clone().stdout).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::run_command::run_command;

    #[test]
    fn runs_command() {
        assert_eq!(run_command("echo test".to_string()), "test");
    }
}