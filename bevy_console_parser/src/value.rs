#[derive(Clone, Debug, PartialEq)]
pub enum ValueRaw<'a> {
    String(String),
    Int(i64, &'a str),
    Float(f64, &'a str),
    Bool(bool, &'a str),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueRawOwned {
    String(String),
    Int(i64, String),
    Float(f64, String),
    Bool(bool, String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl From<ValueRaw<'_>> for ValueRawOwned {
    fn from(value: ValueRaw) -> Self {
        match value {
            ValueRaw::String(s) => ValueRawOwned::String(s),
            ValueRaw::Int(num, raw) => ValueRawOwned::Int(num, raw.to_string()),
            ValueRaw::Float(num, raw) => ValueRawOwned::Float(num, raw.to_string()),
            ValueRaw::Bool(b, raw) => ValueRawOwned::Bool(b, raw.to_string()),
        }
    }
}

impl From<ValueRaw<'_>> for Value {
    fn from(value: ValueRaw) -> Self {
        match value {
            ValueRaw::String(s) => Value::String(s),
            ValueRaw::Int(num, _) => Value::Int(num),
            ValueRaw::Float(num, _) => Value::Float(num),
            ValueRaw::Bool(b, _) => Value::Bool(b),
        }
    }
}

// impl ValueRawOwned {
//     pub fn as_value_raw(&self) -> ValueRaw<'_> {
//         match self {
//             ValueRawOwned::String(s) => ValueRaw::String(s),
//             ValueRawOwned::Int(num, raw) => ValueRaw::Int(num, &raw),
//             ValueRawOwned::Float(num, raw) => ValueRaw::Float(num, &raw),
//             ValueRawOwned::Bool(b, raw) => ValueRaw::Bool(b, &raw),
//         }
//     }
// }

impl From<ValueRawOwned> for Value {
    fn from(value: ValueRawOwned) -> Self {
        match value {
            ValueRawOwned::String(s) => Value::String(s),
            ValueRawOwned::Int(num, _) => Value::Int(num),
            ValueRawOwned::Float(num, _) => Value::Float(num),
            ValueRawOwned::Bool(b, _) => Value::Bool(b),
        }
    }
}
