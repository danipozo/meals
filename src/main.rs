#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_contrib;

#[database("meals")]
struct MealsDbConn(diesel::SqliteConnection);

pub mod models;
pub mod schema;

pub mod api_structs;

mod api_test;

use rocket::http::Status;
use rocket::response::{content, status};
use rocket_contrib::json::Json;

use diesel::prelude::*;
use models::{Ingredient, Recipe};
use schema::ingredient::dsl as ing_dsl;
use schema::ingredient_type::dsl as ingtype_dsl;
use schema::recipe::dsl as rec_dsl;
use schema::uses::dsl as uses_dsl;

use itertools::Itertools;

#[get("/menus")]
fn get_menus(conn: MealsDbConn) -> Result<Json<api_structs::Menu>, status::Custom<String>> {
    Err(status::Custom(
        Status::NotImplemented,
        String::from("Endpoint not implemented"),
    ))
}

#[get("/menus/<id>")]
fn get_menu(
    id: usize,
    conn: MealsDbConn,
) -> Result<Json<api_structs::Menu>, status::Custom<String>> {
    Err(status::Custom(
        Status::NotImplemented,
        String::from("Endpoint not implemented"),
    ))
}

#[get("/ingredients")]
fn get_ingredients(
    conn: MealsDbConn,
) -> Result<Json<Vec<api_structs::Ingredient>>, status::Custom<String>> {
    use schema::ingredient::dsl::*;
    use schema::ingredient_type::dsl::*;

    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));
    let not_found_error = status::Custom(Status::NotFound, String::from("Item not found"));

    let ingredients = ing_dsl::ingredient
        .inner_join(ingredient_type.on(ingtype_dsl::id.eq(ing_dsl::ing_type)))
        .select((
            ing_dsl::id,
            name,
            energy,
            carbohydrates,
            sugar,
            proteins,
            fat,
            type_desc,
        ))
        .load::<api_structs::Ingredient>(&*conn)
        .map_err(|e| match e {
            diesel::NotFound => not_found_error,
            _ => db_error.clone(),
        })?;

    Ok(Json(ingredients))
}

#[get("/ingredients/<id_>")]
fn get_ingredient(
    conn: MealsDbConn,
    id_: usize,
) -> Result<Json<api_structs::Ingredient>, status::Custom<String>> {
    use schema::ingredient::dsl::*;

    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));
    let not_found_error = status::Custom(Status::NotFound, String::from("Item not found"));

    let ing = ingredient
        .find(id_ as i32)
        .inner_join(ingtype_dsl::ingredient_type.on(ingtype_dsl::id.eq(ing_type)))
        .select((
            id,
            name,
            energy,
            carbohydrates,
            sugar,
            proteins,
            fat,
            ingtype_dsl::type_desc,
        ))
        .first::<api_structs::Ingredient>(&*conn)
        .map_err(|e| match e {
            diesel::NotFound => not_found_error,
            _ => db_error.clone(),
        })?;

    Ok(Json(ing))
}

#[get("/recipes")]
fn get_recipes(
    conn: MealsDbConn,
) -> Result<Json<Vec<api_structs::Recipe>>, status::Custom<String>> {
    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));
    let not_found_error = status::Custom(Status::NotFound, String::from("Item not found"));

    let mut recipes = uses_dsl::uses
        .inner_join(rec_dsl::recipe.on(rec_dsl::id.eq(uses_dsl::recipe)))
        .inner_join(ing_dsl::ingredient.on(ing_dsl::id.eq(uses_dsl::ingredient)))
        .inner_join(ingtype_dsl::ingredient_type.on(ingtype_dsl::id.eq(ing_dsl::ing_type)))
        .select((
            (
                rec_dsl::id,
                rec_dsl::name,
                rec_dsl::preparation_time,
                rec_dsl::serves,
                rec_dsl::preparation,
            ),
            (
                ing_dsl::id,
                ing_dsl::name,
                ing_dsl::energy,
                ing_dsl::carbohydrates,
                ing_dsl::sugar,
                ing_dsl::proteins,
                ing_dsl::fat,
                ing_dsl::ing_type,
            ),
            ingtype_dsl::type_desc,
        ))
        .load::<(Recipe, Ingredient, String)>(&*conn)
        .map_err(|e| match e {
            diesel::NotFound => not_found_error,
            _ => db_error.clone(),
        })?;

    recipes.sort_by(|(r1, _, _), (r2, _, _)| r1.id.cmp(&r2.id));

    let recipes = recipes
        .iter()
        .group_by(|(r, _, _)| r)
        .into_iter()
        .map(|(r, group)| {
            let ings = group
                .cloned()
                .into_iter()
                .map(|(_, i, s)| {
                    api_structs::IngredientBuilder::default()
                        .from_db_struct(&i)
                        .ing_type(s)
                        .build()
                        .unwrap()
                })
                .collect();

            api_structs::RecipeBuilder::default()
                .from_db_struct(r)
                .ingredients(ings)
                .build()
                .unwrap()
        })
        .collect();

    Ok(Json(recipes))
}

#[get("/recipes/<id>")]
fn get_recipe(
    conn: MealsDbConn,
    id: usize,
) -> Result<Json<api_structs::Recipe>, status::Custom<String>> {
    let db_error = status::Custom(Status::InternalServerError, String::from("Database error"));
    let not_found_error = status::Custom(Status::NotFound, String::from("Item not found"));

    let recipe_ = rec_dsl::recipe
        .find(id as i32)
        .first::<models::Recipe>(&*conn)
        .map_err(|e| match e {
            diesel::NotFound => not_found_error.clone(),
            _ => db_error.clone(),
        })?;

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
        .map_err(|e| match e {
            diesel::NotFound => not_found_error,
            _ => db_error.clone(),
        })?;

    Ok(Json(
        api_structs::RecipeBuilder::default()
            .from_db_struct(&recipe_)
            .ingredients(ingredients)
            .build()
            .unwrap(),
    ))
}

#[get("/status")]
fn status() -> content::Json<String> {
    content::Json(String::from(r#"{ "status" : "OK" }"#))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(MealsDbConn::fairing()).mount(
        "/",
        routes![
            get_menu,
            get_menus,
            get_recipes,
            get_recipe,
            get_ingredients,
            get_ingredient,
            status
        ],
    )
}

fn main() {
    rocket().launch();
}
