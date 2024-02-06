use std::io;

pub fn read_terminal() -> String{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => input,
        Err(error) => panic!("error: {error}")
    }
}

pub fn read_options() {
	let input = read_terminal().trim().to_string();
	match InputOptions::from(input.clone()) {
		InputOptions::Exit => std::process::exit(0),
		InputOptions::New => todo!(),
		InputOptions::Show => todo!(),
		InputOptions::Link => todo!(),
		InputOptions::Error => {
			println!("----------------------------------------------------");
			println!("Error, option {} not valid", input);
			show_options();
			read_options();
		}
	}
}

pub fn show_options() {
	println!("Choose an option: ");
    println!("{}", InputOptions::Exit.to_string());
    println!("{} <gate_type>", InputOptions::New.to_string());
    println!("{} <gate_id || all>", InputOptions::Show.to_string());
    println!("{} <left || right> <input> <output>", InputOptions::Link.to_string());
}

enum InputOptions {
    Exit, New, Show, Link, Error
}

impl InputOptions {
    pub fn to_string(&self) -> String {
        match self {
            InputOptions::Exit => String::from("Exit"),
            InputOptions::New => String::from("New"),
            InputOptions::Show => String::from("Show"),
            InputOptions::Link => String::from("Link"),
            InputOptions::Error => String::from("Error"),
        }
    }
}

impl From<String> for InputOptions {
    fn from(value: String) -> Self {
		let mut res= value.split_whitespace();
        match res.next() {
			Some("Exit") => InputOptions::Exit,
			Some("New") => InputOptions::New,
			Some("Show") => InputOptions::Show,
			Some("Link") => InputOptions::Link,
			_=> InputOptions::Error
		}
    }
}