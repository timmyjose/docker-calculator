use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub(crate) struct AddInput {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize)]
pub(crate) struct SubInput {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize)]
pub(crate) struct MulInput {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize)]
pub(crate) struct DivInput {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize)]
pub(crate) struct AddOutput {
    pub sum: i32,
}

#[derive(Serialize)]
pub(crate) struct SubOutput {
    pub difference: i32,
}

#[derive(Serialize)]
pub(crate) struct MulOutput {
    pub product: i32,
}

#[derive(Serialize)]
pub(crate) struct DivOutput {
    pub quotient: i32,
}
