use crate::config;
use crate::flow::GitCommand;
use crate::parse_branch_issue_id;
use log::{debug, error, info};
use regex::Regex;
pub struct ViewHandler {
    command: String,
}

impl ViewHandler {
    pub fn new(command: String) -> Self {
        Self { command }
    }
}

pub trait View {
    fn open_pr_web(&self, issue_id: &str);
    fn open_issue_web(&self, issue_id: &str);
    fn open_default_web(&self);
    fn run_command(&self);
}

impl GitCommand for ViewHandler {}

impl View for ViewHandler {
    fn run_command(&self) {
        let config = config::load_config(config::CONFIG_PATH).unwrap();
        let issue_id = parse_branch_issue_id(&config);
        if self.command == "true" {
            self.open_default_web();
        }
        if self.command == "pr" || self.command == "p" {
            self.open_pr_web(&issue_id)
        }
        if self.command == "issue" || self.command == "i" {
            self.open_issue_web(&issue_id);
        }
        info!("打开浏览器目前支持的参数列表: [ pr | p | issue | i ]");
        std::process::exit(0);
    }
    fn open_pr_web(&self, issue_id: &str) {
        let pr_output = self.run_command_and_check_code("gh pr list --json title,number -L 100");
        let json = String::from_utf8_lossy(&pr_output);
        let pr_map: serde_json::Value = serde_json::from_str(&json).unwrap();
        let issue_id_re = Regex::new(r".*#(\d+).*").unwrap();
        //  在 pr_map 中遍历，通过 title找到 issue_id 对应的 pr number
        for pr in pr_map.as_array().unwrap() {
            debug!("issue id {}", issue_id);
            debug!("pr: {:?}", pr);
            debug!("pr number : {:?}", pr["number"].as_i64().unwrap());
            match pr["title"].as_str() {
                Some(title) => {
                    debug!("pr title: {}", title);
                    if issue_id_re.is_match(title) {
                        let issue_number = issue_id_re
                            .captures(title)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str();
                        debug!("issue_number: {}", issue_number);
                        if issue_number == issue_id {
                            let command =
                                format!("gh pr view {} --web", pr["number"].as_i64().unwrap());
                            debug!("command: {}", &command);
                            self.run_command_and_check_code(&command);
                            std::process::exit(0);
                        }
                    }
                }
                None => error!("执行命令 [ gh pr list --json title,number ] 时无法解析出 title"),
            }
        }
        error!("未找到对应的 pr, 请检查是否已经创建 pr");
    }
    fn open_issue_web(&self, issue_id: &str) {
        let command = format!("gh issue view {} --web", issue_id);
        self.run_command_and_check_code(&command);
        std::process::exit(0);
    }
    fn open_default_web(&self) {
        let command = "gh issue list --web".to_string();
        self.run_command_and_check_code(&command);
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_view_pr() {
        use log::LevelFilter;
        let test_str = "view (closed #28)".to_string();
        let re = Regex::new(r".*#(\d+).*").unwrap();
        if re.is_match(&test_str) {
            let s = re
                .captures(&test_str)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .to_string();
            println!("s: {}", s);
        }
        simple_logger::SimpleLogger::new()
            .with_level(LevelFilter::Debug)
            .init()
            .unwrap();
        let handler = ViewHandler::new("pr".to_string());
        handler.run_command();
    }
}
