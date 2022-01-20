use std::process::{Command, ExitStatus};
use std::io::Result;

pub fn execute(exe: &str, args: &[&str]) -> Result<ExitStatus> {
    Command::new(exe).args(args).spawn()?.wait()
}
/*
pub fn app(exe: &str){
    let args :Vec<&str> = vec![];
    match execute(exe: &str, args: &[&str]){
        _=>{}
    }
}
*/