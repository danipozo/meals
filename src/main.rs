#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

#[database("meals")]
struct MealsDbConn(diesel::SqliteConnection);

use rocket_contrib::databases::diesel::Connection;

pub mod models;
pub mod schema;

pub mod api_structs;

use rocket::http::Status;
use rocket::response::{content, status};
use rocket_contrib::json::Json;

use diesel::dsl::*;
use diesel::prelude::*;
use models::{Ingredient, Recipe};
use schema::ingredient::dsl as ing_dsl;
use schema::ingredient_type::dsl as ingtype_dsl;
use schema::recipe::dsl as rec_dsl;
use schema::uses::dsl as uses_dsl;

#[get("/menus")]
fn get_menus() -> content::Json<String> {
    content::Json("{ 'test' : 'test' }".to_string())
}

#[get("/ingredients")]
fn get_ingredients(
    conn: MealsDbConn,
) -> Result<Json<Vec<api_structs::Ingredient>>, status::Custom<String>> {
    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));

    let ingredients = ing_dsl::ingredient
        .inner_join(ingtype_dsl::ingredient_type.on(ingtype_dsl::id.eq(ing_dsl::ing_type)))
        .select((
            ing_dsl::id,
            ing_dsl::name,
            ing_dsl::energy,
            ing_dsl::carbohydrates,
            ing_dsl::sugar,
            ing_dsl::proteins,
            ing_dsl::fat,
            ingtype_dsl::type_desc,
        ))
        .load::<api_structs::Ingredient>(&*conn)
        .map_err(|_e| db_error)?;

    Ok(Json(ingredients))
}

#[get("/ingredients/<id>")]
fn get_ingredient(
    conn: MealsDbConn,
    id: usize,
) -> Result<Json<api_structs::Ingredient>, status::Custom<String>> {
    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));

    let ingredient = ing_dsl::ingredient
        .find(id as i32)
        .inner_join(ingtype_dsl::ingredient_type.on(ingtype_dsl::id.eq(ing_dsl::ing_type)))
        .select((
            ing_dsl::id,
            ing_dsl::name,
            ing_dsl::energy,
            ing_dsl::carbohydrates,
            ing_dsl::sugar,
            ing_dsl::proteins,
            ing_dsl::fat,
            ingtype_dsl::type_desc,
        ))
        .first::<api_structs::Ingredient>(&*conn)
        .map_err(|_e| db_error)?;

    Ok(Json(ingredient))
}

#[get("/recipes/<id>")]
fn get_recipe(
    conn: MealsDbConn,
    id: usize,
) -> Result<Json<api_structs::Recipe>, status::Custom<String>> {
    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));

    let recipe_ = rec_dsl::recipe
        .find(id as i32)
        .first::<models::Recipe>(&*conn)
        .map_err(|_e| db_error.clone())?;

    let ingredients = uses_dsl::uses
        .filter(uses_dsl::recipe.eq(id as i32))
        .inner_join(ing_dsl::ingredient.on(ing_dsl::id.eq(uses_dsl::ingredient)))
        .inner_join(ingtype_dsl::ingredient_type.on(ingtype_dsl::id.eq(ing_dsl::ing_type)))
        .select((
            ing_dsl::id,
            ing_dsl::name,
            ing_dsl::energy,
            ing_dsl::carbohydrates,
            ing_dsl::sugar,
            ing_dsl::proteins,
            ing_dsl::fat,
            ingtype_dsl::type_desc,
        ))
        .load::<api_structs::Ingredient>(&*conn)
        .map_err(|_e| db_error)?;

    Ok(Json(
        api_structs::RecipeBuilder::default()
            .from_db_struct(&recipe_)
            .ingredients(ingredients)
            .build()
            .unwrap(),
    ))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(MealsDbConn::fairing())
        .mount("/", routes![get_recipe, get_ingredients, get_ingredient])
}

fn main() {
    rocket().launch();
}
