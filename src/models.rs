#[derive(Queryable, Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: Option<String>,
}

#[derive(Queryable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub preparation_time: i32,
    pub main_ingredient: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diesel::prelude::*;
    use crate::diesel::Connection;
    use crate::schema::ingredient::dsl::*;

    #[test]
    fn query_ingredients() {
        let conn = diesel::SqliteConnection::establish("test.sqlite")
            .expect("Error connecting to database");

        let _result = ingredient
            .limit(5)
            .load::<Ingredient>(&conn)
            .expect("Error loading ingredients");
    }
}
