table! {
    includes (menu, recipe) {
        menu -> Int4,
        recipe -> Int4,
        day_number -> Int4,
    }
}

table! {
    ingredient (id) {
        id -> Int4,
        name -> Varchar,
        energy -> Nullable<Int4>,
        carbohydrates -> Nullable<Int4>,
        sugar -> Nullable<Int4>,
        proteins -> Nullable<Int4>,
        fat -> Nullable<Int4>,
        ing_type -> Int4,
    }
}

table! {
    ingredient_type (id) {
        id -> Int4,
        type_desc -> Varchar,
    }
}

table! {
    menu (id) {
        id -> Int4,
        description -> Nullable<Text>,
    }
}

table! {
    recipe (id) {
        id -> Int4,
        name -> Varchar,
        preparation_time -> Int4,
        serves -> Int4,
        preparation -> Text,
    }
}

table! {
    units (id) {
        id -> Int4,
        unit -> Nullable<Varchar>,
    }
}

table! {
    uses (recipe, ingredient) {
        recipe -> Int4,
        ingredient -> Int4,
        main_ingredient -> Int4,
        quantity -> Int4,
        unit -> Nullable<Int4>,
    }
}

joinable!(includes -> menu (menu));
joinable!(includes -> recipe (recipe));
joinable!(ingredient -> ingredient_type (ing_type));
joinable!(uses -> ingredient (ingredient));
joinable!(uses -> recipe (recipe));
joinable!(uses -> units (unit));

allow_tables_to_appear_in_same_query!(
    includes,
    ingredient,
    ingredient_type,
    menu,
    recipe,
    units,
    uses,
);
