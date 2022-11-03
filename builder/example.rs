#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use derive_builder::Builder;
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}
pub struct CommandBuilder {
    executable: std::option::Option<String>,
    args: std::option::Option<Vec<String>>,
    env: std::option::Option<Vec<String>>,
    current_dir: Option<String>,
}
impl CommandBuilder {
    pub fn executable(&mut self, executable: String) -> &mut Self {
        self.executable = std::option::Option::Some(executable);
        self
    }
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.args = std::option::Option::Some(args);
        self
    }
    pub fn env(&mut self, env: Vec<String>) -> &mut Self {
        self.env = std::option::Option::Some(env);
        self
    }
    pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
        self.current_dir = std::option::Option::Some(current_dir);
        self
    }
    pub fn build(&self) -> std::result::Result<Command, std::boxed::Box<dyn std::error::Error>> {
        std::result::Result::Ok(Command {
            executable: self.executable.clone().ok_or("executable is not set")?,
            args: self.args.clone().ok_or("args is not set")?,
            env: self.env.clone().ok_or("env is not set")?,
            current_dir: self.current_dir.clone(),
        })
    }
}
impl Command {
    fn builder() -> CommandBuilder {
        CommandBuilder {
            executable: std::option::Option::None,
            args: std::option::Option::None,
            env: std::option::Option::None,
            current_dir: std::option::Option::None,
        }
    }
}
fn main() {}