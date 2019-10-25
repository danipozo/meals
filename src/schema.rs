table! {
    includes (menu, recipe) {
        menu -> Integer,
        recipe -> Integer,
        day_number -> Integer,
    }
}

table! {
    ingredient (id) {
        id -> Integer,
        name -> Text,
        energy -> Nullable<Integer>,
        carbohydrates -> Nullable<Integer>,
        sugar -> Nullable<Integer>,
        proteins -> Nullable<Integer>,
        fat -> Nullable<Integer>,
        ing_type -> Integer,
    }
}

table! {
    ingredient_type (id) {
        id -> Integer,
        type_desc -> Text,
    }
}

table! {
    menu (id) {
        id -> Integer,
        description -> Nullable<Text>,
    }
}

table! {
    recipe (id) {
        id -> Integer,
        name -> Text,
        preparation_time -> Integer,
        serves -> Integer,
        preparation -> Text,
    }
}

table! {
    units (id) {
        id -> Integer,
        unit -> Nullable<Text>,
    }
}

table! {
    uses (recipe, ingredient) {
        recipe -> Integer,
        ingredient -> Integer,
        main_ingredient -> Integer,
        quantity -> Integer,
        unit -> Nullable<Integer>,
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
