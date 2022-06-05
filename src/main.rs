use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::process::Command;
use std::result::Result as CResult;
use std::str;
#[macro_use]
extern crate colour;

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
pub struct Args {
    /// custom commit message
    #[clap(short = 'm', long, required = false, default_value = "false")]
    message: String,

    /// just print command without exec
    #[clap(
        short = 'p',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    print: String,

    /// Open issue on chrome
    #[clap(
        short = 'w',
        long,
        takes_value = false,
        forbid_empty_values = false,
        required = false,
        default_missing_value = "true",
        default_value = "false"
    )]
    web: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    number: usize,
    title: String,
}

fn get_branch() -> CResult<Vec<u8>, Box<std::io::Error>> {
    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .output()?;
    Ok(branch.stdout)
}

fn is_element_in_vec(a: &usize, v: &Vec<usize>) -> bool {
    let result: bool = false;
    for n in v.iter() {
        if a == n {
            let result: bool = true;
            return result;
        }
    }
    result
}

fn lower_and_bug_head_replace(source: &str) -> String {
    let lower_source = source.to_lowercase();
    let bug_re: Regex = Regex::new(r"bug$").unwrap();
    if bug_re.is_match(&lower_source) {
        let result: String = lower_source.replace("bug", "bugfix");
        result
    } else {
        lower_source
    }
}

// fn load_json (json: &str) -> Result<Vec<Foo>> {

//     let load_json = serde_json::from_str(json)?;
//     Ok(load_json)
// }

fn main() {
    let args = Args::parse();
    if args.web == "true" {
        if let Ok(result) = Command::new("gh").args(["issue", "list", "--web"]).output() {
            let code = result.status.code();
            if code != Some(0) {
                println!("{}", str::from_utf8(&result.stdout).unwrap());
            } else {
                std::process::exit(1);
            }
        }
    }
    if let Ok(result) = Command::new("gh")
        .args(["issue", "list", "--json", "number,title"])
        .output()
    {
        if let Ok(d) = str::from_utf8(&result.stdout) {
            let mut b_n_vec: Vec<usize> = Vec::new();
            let load_json: Vec<Foo> = serde_json::from_str(&d).unwrap();
            if let Ok(result) = get_branch() {
                let branch = String::from_utf8_lossy(&result);
                let issue_re: Regex = Regex::new(r".*issue#?(\d+).*").unwrap();
                if issue_re.is_match(&branch) {
                    let re_result = issue_re.captures(&branch).unwrap();
                    let branch_number = &re_result[1];
                    // println!("{:?}", branch_number);
                    let usize_branch = branch_number.parse::<usize>().unwrap();
                    for issue in load_json.iter() {
                        b_n_vec.push(issue.number);
                    }
                    if is_element_in_vec(&usize_branch, &b_n_vec) {
                        for issue in load_json.iter() {
                            if issue.number == usize_branch {
                                let title = &issue.title;
                                let title_re: Regex = Regex::new(r"\[(.*)\] (.*)").unwrap();
                                if title_re.is_match(title) {
                                    let title_result = title_re.captures(&title).unwrap();
                                    let tag = &title_result[1];

                                    let mut message: &str = &String::new();
                                    if args.message != "false" {
                                        message = &args.message;
                                    } else {
                                        message = &title_result[2];
                                    }

                                    let currect_tag = lower_and_bug_head_replace(tag);
                                    let c = format!(
                                        "git commit -m \"{}: {} (closed #{})(wf -l)\"",
                                        currect_tag, message, issue.number
                                    );
                                    let d = format!(
                                        "{}: {} (closed #{})(wf -l)",
                                        currect_tag, message, issue.number
                                    );
                                    if args.print == "true" {
                                        println!("{}", c);
                                        std::process::exit(1);
                                    }
                                    if let Ok(result) =
                                        Command::new("git").args(["commit", "-m", &d]).output()
                                    {
                                        match result.status.code() {
                                            Some(code) => {
                                                if code == 0 {
                                                    let cc = String::from_utf8_lossy(&result.stdout);
                                                    println!(" {}\n {}\n ", &c, &cc);
                                                    green!(" - success\n")
                                                } else {
                                                    let cc = String::from_utf8_lossy(&result.stderr);
                                                    println!(" {}\n {}\n", &c, &cc);
                                                    red!(" - failed\n")
                                                }
                                            }
                                            None => {
                                                println!("提交失败, 未获取对应返回码");
                                                std::process::exit(1);
                                            }
                                        }
                                    } else {
                                        println!("{} failed !", c);
                                    }
                                } else {
                                    println!("miss title re");
                                    std::process::exit(1);
                                }
                            }
                        }
                    } else {
                        println!("branch number not in issue list ");
                        std::process::exit(1);
                    }
                } else {
                    red!(
                        "branch -> {:?} 不符合匹配规则 like: (*issue#666*|*issue666*)\n",
                        branch
                    );
                    std::process::exit(1);
                }
            } else {
                println!("{:?}", result)
            }
        }
    } else {
        red!("gh command failed!")
    }
}
