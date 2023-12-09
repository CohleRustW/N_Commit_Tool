use crate::config::Config;
use crate::flow::GitCommand;
use crate::get_target_issue;
use crate::view::{ViewHandler, View};
pub struct Pr {
    config: Config,
    all_branch: Vec<String>,
    biggerst_branch_name: String,
    fork_remote: String,
    force: bool,
}
use log::{debug, error, info, warn};

impl GitCommand for Pr {}

impl Pr {
    pub fn new(c: String, config: &Config, force: bool, chooise: String) -> Self {
        match &config.fork_remote_name {
            Some(fork_remote) => {
                debug!("fork_remote_name is {}", fork_remote);
                let chooise = chooise == "true";
                let (_, branch_name, all_branch) = get_target_issue(
                    chooise,
                    &config.version_compare_re,
                    config.enable_auto_fetch.clone(),
                    &fork_remote,
                    &config.remote_branch_name_template,
                    false,
                );
                Self {
                    config: config.clone(),
                    all_branch,
                    biggerst_branch_name: branch_name,
                    fork_remote: fork_remote.clone(),
                    force: force,
                }
            }
            None => {
                error!("fork_remote_name is not set in config file");
                std::process::exit(1);
            }
        }
    }
    // choose the branch to push fork
    pub fn choose_remote_fork(&self) -> String {
        match &self.config.fork_remote_name {
            Some(fork_remote_name) => fork_remote_name.clone(),
            None => {
                error!("fork_remote_name is not set in config file");
                std::process::exit(1);
            }
        }
    }
}

pub trait PrCommand {
    fn pr(&self);
}

impl PrCommand for Pr {
    fn pr(&self) {
        let fork_remote_name = self.choose_remote_fork();
        // 检查 remote 是否包括当前的分支
        let current_branch_out = self.run_command_and_check_code("git rev-parse --abbrev-ref HEAD");
        let current_branch = std::string::String::from_utf8(current_branch_out).unwrap();
        //  如果不在远端就推送
        debug!("current_branch: {}", current_branch);
        let mut push_cmd = format!("git push {} {}", fork_remote_name, current_branch);
        if self.force {
            push_cmd = format!("git push {} {} -f", fork_remote_name, current_branch);
        }
        debug!(
            "{} not contains in {:#?}, push_cmd: {}",
            &current_branch, &self.all_branch, push_cmd
        );
        let result = self.run_command_and_check_code(&push_cmd);
        debug!("push result: {:#?}", result);
        println!("{}", std::string::String::from_utf8(result).unwrap());
        // 推送后执行 gh 命令 创建 pr
        let pr_cmd = format!(
            "gh pr create -f -H {} -B {} --body 'asdfasf'",
            current_branch, self.biggerst_branch_name
        );
        debug!("execute pr_cmd: {}", pr_cmd);
        let pr_result = self.run_command_and_check_code(&pr_cmd);
        println!("{}", std::string::String::from_utf8(pr_result).unwrap());
        let view = ViewHandler::new("pr".to_string());
        view.run_command();
        std::process::exit(0);
    }
}
