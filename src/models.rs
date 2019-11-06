use crate::schema::{ingredient, recipe, uses};

#[derive(Queryable, Clone)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub energy: Option<i32>,
    pub carbohydrates: Option<i32>,
    pub sugar: Option<i32>,
    pub proteins: Option<i32>,
    pub fat: Option<i32>,
    pub ing_type: i32,
}

#[derive(Queryable, PartialEq, Clone)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub preparation_time: i32,
    pub serves: i32,
    pub preparation: String,
}

#[derive(Queryable)]
pub struct Uses {
    pub recipe: i32,
    pub ingredient: i32,
}

#[derive(Queryable)]
pub struct Menu {
    pub id: i32,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diesel::dsl::*;
    use crate::diesel::prelude::*;
    use crate::diesel::Connection;
    use diesel_migrations::run_pending_migrations;

    #[test]
    fn query_ingredients() {
        use crate::schema::ingredient::dsl::*;

        let conn =
            diesel::SqliteConnection::establish(":memory:").expect("Error connecting to database");

        run_pending_migrations(&conn).expect("Error running migrations");

        insert_into(ingredient)
            .values((name.eq("Test ingredient"), ing_type.eq(1)))
            .execute(&conn)
            .expect("Error inserting ingredient");

        let _result = ingredient
            .limit(5)
            .load::<Ingredient>(&conn)
            .expect("Error loading ingredients");
    }

    #[test]
    fn query_recipes() {
        use crate::schema::recipe::dsl::*;

        let conn =
            diesel::SqliteConnection::establish(":memory:").expect("Error connecting to database");

        run_pending_migrations(&conn).expect("Error running migrations");

        insert_into(recipe)
            .values((
                name.eq("Test ingredient"),
                preparation_time.eq(20),
                serves.eq(2),
                preparation.eq("Preparation text"),
            ))
            .execute(&conn)
            .expect("Error inserting ingredient");

        let _result = recipe
            .limit(5)
            .load::<Recipe>(&conn)
            .expect("Error loading ingredients");
    }
}
