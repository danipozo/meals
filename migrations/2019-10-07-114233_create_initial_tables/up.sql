create table if not exists ingredient_type (
  id serial primary key not null,
  type_desc varchar(50) not null
);

create table if not exists ingredient (
  id serial primary key not null,
  name varchar(255) not null,
  energy int,
  carbohydrates int,
  sugar int,
  proteins int,
  fat int,
  ing_type int not null,

  foreign key (ing_type) references ingredient_type(id)
);

insert into ingredient_type(id, type_desc) values (1, 'Carne'), (2, 'Pescado'), (3, 'Pasta'), (4, 'Legumbre'),
                                             (5, 'Fruta'), (6, 'Verdura'), (7, 'Condimento'), (8, 'Fruto seco'),
					     (9, 'Graso'), (255, 'Otro')
on conflict do nothing;

create table if not exists recipe (
   id serial primary key not null,
   name varchar(255) not null,
   preparation_time int not null check (0 < preparation_time), -- expressed in minutes
   serves int not null check (0 < serves),
   preparation text not null
);

create table if not exists units (
  id serial primary key not null,
  unit varchar(10)
);

insert into units(id, unit) values (1, 'gr'), (2, 'ml'), (3, 'unidad') on conflict do nothing;

create table if not exists uses (
  recipe integer not null unique,
  ingredient integer not null unique,
  main_ingredient int not null check (main_ingredient = 0 or main_ingredient = 1), -- whether this is a main ingredient of the recipe

  quantity int not null,
  unit int,

  primary key (recipe, ingredient),
  foreign key (recipe) references recipe(id),
  foreign key (ingredient) references ingredient(id),
  foreign key (unit) references units(id)
);

create table if not exists menu (
  id serial primary key not null,
  description text
);

create table if not exists includes (
  menu integer not null,
  recipe integer not null,
  day_number int not null,

  foreign key (menu) references menu(id),
  foreign key (recipe) references recipe(id),
  primary key (menu, recipe)
);
