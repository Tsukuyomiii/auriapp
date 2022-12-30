use std::io::{stdin, Read};
use std::collections::HashMap;

fn main() {
    let mut cmd_list = CommandList::new();
    cmd_list.list_commands();
}

/// hello old friend
pub fn get_user_string() -> String {
    let mut stdin = stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).expect("get_user_string() failed at stdin.read_to_string(&mut buffer)");
    buffer
}

type CommandProc = Box<dyn Fn() -> ()>;

struct Command {
    desc: String,
    proc: CommandProc,
}

impl Command {
    pub fn new(proc: CommandProc, desc: String) -> Self {
        Self { proc, desc }
    }

    pub fn execute(&self) {
        (self.proc)();
    }
}

impl Default for Command {
    fn default() -> Self {
        Self {
            proc: Box::new(|| println!("default fn")),
            desc: "The default command proc.".to_string()
        }
    }
}

struct CommandList(HashMap<String, Command>);

impl CommandList {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn list_commands(&self) {
        if self.0.len() == 0 { 
            println!("There are no commands registered.");
            return; 
        }

        for (k, cmd) in self.0.iter() {
            println!("{}: {}", k, cmd.desc);
        }
    }
}
