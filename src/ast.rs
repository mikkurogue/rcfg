use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub decorators: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Serialize)]
pub enum Type {
    String,
    Int,
    Bool,
    Array(Box<Type>),
    Custom(String),
}

