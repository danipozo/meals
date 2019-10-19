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
        type_desc -> Nullable<Text>,
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

joinable!(ingredient -> ingredient_type (ing_type));
joinable!(uses -> ingredient (ingredient));
joinable!(uses -> recipe (recipe));
joinable!(uses -> units (unit));

allow_tables_to_appear_in_same_query!(
    ingredient,
    ingredient_type,
    recipe,
    units,
    uses,
);
