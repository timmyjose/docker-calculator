mod schema;

use actix_web::{post, web::Json, Result};
use schema::{AddInput, AddOutput, DivInput, DivOutput, MulInput, MulOutput};

use crate::schema::{SubInput, SubOutput};

#[post("/add")]
pub async fn handle_add(input: Json<AddInput>) -> Result<Json<AddOutput>> {
    Ok(Json(AddOutput {
        sum: input.x + input.y,
    }))
}

#[post("/sub")]
pub async fn handle_sub(input: Json<SubInput>) -> Result<Json<SubOutput>> {
    Ok(Json(SubOutput {
        difference: input.x - input.y,
    }))
}

#[post("/mul")]
pub async fn handle_mul(input: Json<MulInput>) -> Result<Json<MulOutput>> {
    Ok(Json(MulOutput {
        product: input.x * input.y,
    }))
}

#[post("/div")]
pub async fn handle_div(input: Json<DivInput>) -> Result<Json<DivOutput>> {
    Ok(Json(DivOutput {
        quotient: if input.y == 0 { 0 } else { input.x / input.y },
    }))
}
