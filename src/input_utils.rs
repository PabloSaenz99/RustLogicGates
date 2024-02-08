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
    Exit, New(GateTypes), Show(String), Link(String, u128, u128), Unlink(String, u128, u128), Input(u128, String, bool), Error
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
			Some("Unlink") => {
                let mut opts = Vec::with_capacity(2);
                while let Some(opt)= res.next() {
                    opts.push(opt.to_string());
                }
                if opts.len() < 2 || opts.len() > 3 {
                    return InputOptions::Error
                }
                else {
                    if opts.len() == 2 {
                        opts.push("0".to_string())
                    }
                    return InputOptions::Unlink(opts[0].clone(), opts[1].clone().parse().unwrap_or(0), opts[2].clone().parse().unwrap_or(0))
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

pub fn error_option(input: &str, ctrl: &mut Controller) {
    println!("----------------------------------------------------");
    println!("Error, option {} not valid", input);
    show_options();
    ctrl.parse_options();
}

pub fn show_options() {
	println!("Choose an option:");
    println!("Exit");
    println!("New <gate_type>");
    println!("Show <gate_id || all>");
    println!("Link <left || right> <from> <to>");
    println!("Unlink <left || right || none> <from> <to | none>");
    println!("Input <gate_id> <left || right>, <bool>");
}