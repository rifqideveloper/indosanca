use std::process::{Command, ExitStatus};
use std::io::Result;

fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}
pub app(exe: &str){
    let args :vec = vec![];
    match execute(exe: &str, args: &[&str]){
        _=>{}
    }
}