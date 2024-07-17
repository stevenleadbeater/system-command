use crate::run_command::RunCommandTrait;

#[derive(Clone)]
pub struct MockRunCommand {
    pub result: Vec<String>,
}

impl RunCommandTrait for MockRunCommand {
    fn run_command(&mut self, _command: String) -> String {
        if self.result.len() > 0 {
            self.result.remove(0).clone()
        } else {
            "".to_string()
        }
    }
}