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
