create table if not exists ingredient (
  id integer primary key not null,
  name varchar(255)
);

create table if not exists recipe (
   id integer primary key not null,
   name varchar(255),
   preparation_time int not null check (0 < preparation_time), -- expressed in minutes
   main_ingredient int,

   foreign key (main_ingredient) references ingredient(id)
);

create table if not exists uses (
  recipe int not null,
  ingredient int not null,

  foreign key (recipe) references recipe(id),
  foreign key (ingredient) references ingredient(id),
  primary key (recipe, ingredient)
);
