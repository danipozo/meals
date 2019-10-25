extern crate derive_builder;
extern crate serde;

use derive_builder::Builder;
use serde::Serialize;

#[derive(Queryable, Serialize, Clone, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub energy: Option<i32>,
    pub carbohydrates: Option<i32>,
    pub sugar: Option<i32>,
    pub proteins: Option<i32>,
    pub fat: Option<i32>,
    pub ing_type: String,
}

#[derive(Serialize, Builder, Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub preparation_time: i32,
    pub serves: i32,
    pub preparation: String,
    pub ingredients: Vec<Ingredient>,
}

use crate::models;

impl RecipeBuilder {
    pub fn from_db_struct(&mut self, r: &models::Recipe) -> &mut RecipeBuilder {
        self.id(r.id)
            .name(r.name.clone())
            .preparation_time(r.preparation_time)
            .serves(r.serves)
            .preparation(r.preparation.clone())
    }
}
