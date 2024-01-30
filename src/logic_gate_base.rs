pub struct LogicGate {
    name: String,
    left_input: bool,
    right_input: bool,
    output: bool,

    left_input_connection: Option<Box<LogicGate>>,
    right_input_connection: Option<Box<LogicGate>>,
    output_connection: Option<Box<Vec<LogicGate>>>,

    
}

pub trait LogicGateOperations<T> {
    fn new() -> T;
    fn calculate_output(&mut self) -> bool;
}


impl LogicGate {
    //Default constructor
    pub fn new(name: String) -> LogicGate {
        LogicGate {
            name,
            left_input: false, right_input: false, output: false,
            left_input_connection: None,
            right_input_connection: None,
            output_connection: None
        }
    }

    pub fn get_name(&self) -> &String { &self.name }
    pub fn get_left_input(&self) -> bool { self.left_input }
    pub fn get_right_input(&self) -> bool { self.right_input }
    pub fn get_output(&self) -> bool { self.output }
    pub fn get_string(&self) -> [String; 3] {
        return [self.get_output().to_string(), self.get_name().to_string(), self.get_left_input().to_string() + " - " + &self.get_right_input().to_string()]
    }

    pub fn set_left_input(&mut self, new_input: bool) { self.left_input = new_input; }
    pub fn set_right_input(&mut self, new_input: bool) { self.right_input = new_input; }
    pub fn set_output(&mut self, new_output: bool) { self.output = new_output; }

    pub fn get_left_input_connection(&self) -> &Option<Box<LogicGate>> { &self.left_input_connection }
    pub fn get_right_input_connection(&self) -> &Option<Box<LogicGate>> { &self.right_input_connection }
    pub fn get_output_connection(&self) -> &Option<Box<Vec<LogicGate>>> { &self.output_connection }

    pub fn set_left_input_connection(&mut self, new_input: Box<LogicGate>) { self.left_input_connection = Some(new_input); }
    pub fn set_right_input_connection(&mut self, new_input: Box<LogicGate>) { self.right_input_connection = Some(new_input); }
    pub fn set_output_connection(&mut self, new_output: Box<Vec<LogicGate>>) { self.output_connection = Some(new_output); }
}