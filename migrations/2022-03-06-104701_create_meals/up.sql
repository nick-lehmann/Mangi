CREATE TABLE mensa (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255)
);

CREATE TABLE canteens (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  city VARCHAR(255) NOT NULL,
  address VARCHAR(255) NOT NULL,
  url VARCHAR(255), 
  menu VARCHAR(255),
  mensa INTEGER NOT NULL,
  constraint canteen_mensa foreign key (mensa) references mensa (id)
);

CREATE TABLE meals (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  category VARCHAR(255),
  price_student REAL NOT NULL,
  price_employee REAL NOT NULL,
  notes text,
  image VARCHAR(255),
  url VARCHAR(255), 
  day DATE NOT NULL,
  canteen INTEGER NOT NULL,
  constraint meals_canteen foreign key (canteen) references canteens (id)
);