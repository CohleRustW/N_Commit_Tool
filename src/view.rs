use crate::flow::GitCommand;
use crate::parse_branch_issue_id;
use crate::config;
use log::LevelFilter;
struct ViewHandler {
    command: String,
}

impl ViewHandler {
    fn new(command: String) -> Self {
        Self { command }
    }
}

trait View {
    fn open_pr_web(&self, issue_id: &str);
    fn open_issue_web(&self, issue_id: &str);
    fn open_default_web(&self);
    fn parse_command(&self);
}

impl GitCommand for ViewHandler {}

impl View for ViewHandler {
    fn parse_command(&self) {
        let config = config::load_config(config::CONFIG_PATH).unwrap();
        let issue_id = parse_branch_issue_id(&config);
        if self.command == "pr" {
            self.open_pr_web(&issue_id)
        }
    }
    fn open_pr_web(&self, issue_id: &str) {
        let pr_output = self.run_command_and_check_code("gh pr list --json title,number -L 100");
        let json =  String::from_utf8_lossy(&pr_output);
        let pr_map: serde_json::Value = serde_json::from_str(&json).unwrap();
        println!("{:#?} - {}", pr_map, issue_id)
    }
    fn open_issue_web(&self, issue_id: &str) {
        let command = format!("gh issue view {} --web", issue_id);
        self.run_command_and_check_code(&command);
    }
    fn open_default_web(&self) {
        let command = "gh issue list --web".to_string();
        self.run_command_and_check_code(&command);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_view_pr() {
        simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
        let handler = ViewHandler::new("pr".to_string());
        handler.parse_command();
    }
}