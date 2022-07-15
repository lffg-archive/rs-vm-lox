/// Value representation.
#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(f64),
}

impl Value {
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Bool(_) => "bool",
            Value::Number(_) => "number",
        }
    }
}
