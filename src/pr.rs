use crate::config::{get_current_path, Config};
use crate::flow::GitCommand;
use crate::get_target_issue;
use crate::view::{View, ViewHandler};
pub struct Pr {
    config: Config,
    all_branch: Vec<String>,
    biggerst_branch_name: String,
    fork_remote: String,
    force: bool,
}
use log::{debug, error, info, warn};

impl GitCommand for Pr {}
const PR_MD_TEMPLATE: &str = ".github/PULL_REQUEST_TEMPLATE.md";

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
    fn get_pr_body(&self) -> String;
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
        let git_path = self.get_pr_body();
        // 不同的操作系统的路径分隔符不一样，这里判断下编译的操作系统， 然后拼接
        let format_path = std::path::Path::new(&git_path);
        let pr_md_path = format_path.join(PR_MD_TEMPLATE);
        debug!("pr_md_path: {:?}", pr_md_path);
        // 判断文件是否存在，如果存在就读取文件内容
        let pr_body_path = if pr_md_path.exists() {
            pr_md_path
        } else {
            std::path::Path::new(PR_MD_TEMPLATE).to_path_buf()
        };

        let pr_cmd = format!(
            "gh pr create -f -H {} -B {} -F {}",
            current_branch, self.biggerst_branch_name, pr_body_path.display()
        );
        info!("execute pr_cmd: {}", pr_cmd);
        let pr_result = self.run_command_and_check_code(&pr_cmd);
        println!("{}", std::string::String::from_utf8(pr_result).unwrap());
        let view = ViewHandler::new("pr".to_string());
        view.run_command();
        std::process::exit(0);
    }

    fn get_pr_body(&self) -> String {
        let git_path = get_current_path();
        match git_path {
            Ok(path) => path.to_string(),
            Err(e) => {
                error!("get git path error: {}", e);
                std::process::exit(1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_github_md () {
        let current_git_path = ".github/PULL_REQUEST_TEMPLATE.md";
        // 判断文件是否存在，如果存在就读取文件内容
        let pr_body = std::fs::read_to_string(current_git_path).unwrap();
        println!("{}", pr_body);
    }
}