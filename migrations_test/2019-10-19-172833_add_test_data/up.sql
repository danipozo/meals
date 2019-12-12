-- INGREDIENTS
--
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (1, 'Cebolla', 26, 5, null, 1, 0, 6);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (2, 'Tomate', 19, 4, null, 1, 0, 5);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (3, 'Pimiento rojo', 29, 5, null, 1, 0, 6);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (4, 'Carne de cerdo', 124, 0, null, 25, 5, 1);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (5, 'Arroz', 393, 85, null, 8, 0, 4);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (6, 'Orégano', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (7, 'Tomillo', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (8, 'Cúrcuma', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (9, 'Albahaca', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (10, 'Nuez moscada, en polvo', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (11, 'Sal', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (12, 'Pimentón picante', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (13, 'Pimienta negra', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (14, 'Curry', null, null, null, null, null, 7);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (15, 'Pollo, pechuga', 146, 0, 0, 22, 6, 1);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (16, 'Mango', 59, 13, 13, 1, 0, 5);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (17, 'Uva', 69, 18, 16, 0, 1, 5);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (18, 'Harina de trigo', 364, 76, 0, 1, 0, 255);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (19, 'Mantequilla', 717, 0, 0, 0, 81, 9);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (20, 'Aceite de oliva', 884, 0, 0, 0, 81, 9);
insert into ingredient(id, name, energy, carbohydrates, sugar, proteins, fat, ing_type) values (21, 'Rúcula', 31, 4, 2, 3, 0, 255);

-- RECIPES
--
insert into recipe(id, name, preparation_time, serves, preparation) values (1, 'Arroz 1', 90, 4, '');
insert into recipe(id, name, preparation_time, serves, preparation) values (2, 'Pollo al curry con fruta', 30, 4, '');
insert into recipe(id, name, preparation_time, serves, preparation) values (3, 'Croquetas de atún', 60, 4, '');
insert into recipe(id, name, preparation_time, serves, preparation) values (4, 'Arepas 1', 60, 4, '');
insert into recipe(id, name, preparation_time, serves, preparation) values (5, 'Ensalada de salmón', 20, 2, '');
insert into recipe(id, name, preparation_time, serves, preparation) values (6, 'Pizza', 45, 4, '');

-- INGREDIENTS-RECIPES ASSOCIATIONS
--
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 2, 0, 7, 3);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 3, 0, 2, 3);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 5, 1, 500, 1);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 1, 0, 1, 3);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 4, 0, 400, 1);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 6, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 7, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 8, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 9, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 10, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 11, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 12, 0, 0, null);
insert into uses (recipe, ingredient, main_ingredient, quantity, unit) values (1, 13, 0, 0, null);
