/// Value representation.
#[derive(Debug, Clone, enum_as_inner::EnumAsInner)]
pub enum Value {
    Bool(bool),
    Number(f64),
}
