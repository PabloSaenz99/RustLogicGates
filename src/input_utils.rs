use std::io;

use crate::{controller::Controller, gates::GateTypes};

pub fn read_terminal() -> String {
    println!();
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("error: {error}")
    }
}

pub enum InputOptions {
    Exit, New(GateTypes), Show(String), Link(String, u128, u128), Input(u128, String, bool), Error
}

impl InputOptions {
    pub fn to_string(&self) -> String {
        match self {
            InputOptions::Exit => String::from("Exit"),
            InputOptions::New(_) => String::from("New"),
            InputOptions::Show(_) => String::from("Show"),
            InputOptions::Link(_, _, _) => String::from("Link"),
            InputOptions::Input(_, _, _) => String::from("Input"),
            InputOptions::Error => String::from("Error"),
        }
    }
}

impl From<String> for InputOptions {
    fn from(value: String) -> Self {
		let mut res= value.split_whitespace();
        match res.next() {
			Some("Exit") => InputOptions::Exit,
			Some("New") => {
                match res.next() {
                    Some(_type) => InputOptions::New(GateTypes::from(_type.to_string())),
                    None => InputOptions::Error,
                }
            },
			Some("Show") => {
                match res.next() {
                    Some("all") => InputOptions::Show("all".to_string()),
                    Some(_opt) => InputOptions::Show(_opt.to_string()),
                    None => InputOptions::Error,
                }
            },
			Some("Link") => {
                let mut opts = Vec::with_capacity(3);
                while let Some(opt)= res.next() {
                    opts.push(opt.to_string());
                }
                if opts.len() != 3 {
                    return InputOptions::Error
                }
                else {
                    return InputOptions::Link(opts[0].clone(), opts[1].clone().parse().unwrap_or(0), opts[2].clone().parse().unwrap_or(0))
                }
            },
            Some("Input") => {
                let mut opts = Vec::with_capacity(3);
                while let Some(opt)= res.next() {
                    opts.push(opt.to_string());
                }
                if opts.len() != 3 {
                    return InputOptions::Error
                }
                else {
                    return InputOptions::Input(opts[0].clone().parse().unwrap_or(0), opts[1].clone(), opts[2].clone().parse().unwrap_or(false))
                }
            }
			_=> InputOptions::Error,
		}
    }
}

pub fn error_option(input: String, ctrl: &mut Controller) {
    println!("----------------------------------------------------");
    println!("Error, option {} not valid", input);
    show_options();
    ctrl.parse_options();
}

pub fn show_options() {
	println!("Choose an option: ");
    println!("{}", InputOptions::Exit.to_string());
    println!("{} <gate_type>", InputOptions::New(GateTypes::AND).to_string());
    println!("{} <gate_id || all>", InputOptions::Show(String::new()).to_string());
    println!("{} <left || right> <input> <output>", InputOptions::Link(String::new(), 0, 0).to_string());
    println!("{} <gate_id> <left || right>, <bool>", InputOptions::Input(0, String::new(), false).to_string())
}