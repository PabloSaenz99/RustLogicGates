use crate::gates::{calculate_output, GateTypes};

pub struct LogicGate {
    _type: GateTypes,
    left_input: bool,
    right_input: bool,
    // output: bool,

    left_input_connection: Option<Box<LogicGate>>,
    right_input_connection: Option<Box<LogicGate>>,
    output_connection: Option<Box<Vec<LogicGate>>>,
}

impl LogicGate {
    //Default constructor
    pub fn new(_type: GateTypes) -> LogicGate {
        LogicGate {
            _type,
            left_input: false, right_input: false, //output: false,
            left_input_connection: None,
            right_input_connection: None,
            output_connection: None
        }
    }

    pub fn get_type(&self) -> &GateTypes { &self._type }
    pub fn get_left_input(&self) -> bool { self.left_input }
    pub fn get_right_input(&self) -> bool { self.right_input }
    pub fn get_output(&self) -> bool { calculate_output(&self) }
    pub fn get_string(&self) -> [String; 3] {
        [self.get_output().to_string(),
            self.get_type().to_string(),
            self.get_left_input().to_string() + " - " + &self.get_right_input().to_string()]
    }

    pub fn set_left_input(&mut self, new_input: bool) {
        match &self.left_input_connection {
            Some(a) => {
                println!("Error, input setted, using its value");
                self.left_input = a.get_left_input();
            }
            None => self.left_input = new_input
        }
    }
    pub fn set_right_input(&mut self, new_input: bool) {
        match &self.right_input_connection {
            Some(a) => {
                println!("Error, input setted, using its value");
                self.left_input = a.get_left_input()
            }
            None => self.left_input = new_input
        }
    }

    pub fn get_left_input_connection(&self) -> &Option<Box<LogicGate>> { &self.left_input_connection }
    pub fn get_right_input_connection(&self) -> &Option<Box<LogicGate>> { &self.right_input_connection }
    pub fn get_output_connection(&self) -> &Option<Box<Vec<LogicGate>>> { &self.output_connection }

    pub fn  set_left_input_connection<'a>(&mut self, new_input: Option<Box<LogicGate>>) {
        self.left_input_connection = new_input;
        self.set_left_input(self.left_input);
    }
    pub fn set_right_input_connection(&mut self, new_input: Option<Box<LogicGate>>) {
        self.right_input_connection = new_input;
        self.set_right_input(self.right_input);
    }
    pub fn set_output_connection(&mut self, new_output: Option<Box<Vec<LogicGate>>>) { self.output_connection = new_output; }
}