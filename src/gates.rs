#[derive(Debug)]
pub  enum GateTypes {AND, OR, NOT}

impl GateTypes {
	pub fn to_string(&self) -> String {
		match self {
			GateTypes::AND => "AND".to_string(),
			GateTypes::OR => "OR".to_string(),
			GateTypes::NOT => "NOT".to_string(),
		}
	}
}

impl From<String> for GateTypes {
    fn from(value: String) -> Self {
		match value.as_str() {
			"AND" => GateTypes::AND,
			"OR" => GateTypes::OR,
			"NOT" => GateTypes::NOT,
			_ => panic!("gate type {} does not exists!", value)
		}
	}
}