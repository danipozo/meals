table! {
    ingredient (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}

table! {
    recipe (id) {
        id -> Integer,
        name -> Nullable<Text>,
        preparation_time -> Integer,
        main_ingredient -> Nullable<Integer>,
    }
}

table! {
    uses (recipe, ingredient) {
        recipe -> Integer,
        ingredient -> Integer,
    }
}

joinable!(recipe -> ingredient (main_ingredient));
joinable!(uses -> ingredient (ingredient));
joinable!(uses -> recipe (recipe));

allow_tables_to_appear_in_same_query!(
    ingredient,
    recipe,
    uses,
);
