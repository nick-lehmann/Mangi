CREATE TABLE mensa (id SERIAL PRIMARY KEY, name VARCHAR(255));
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
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  telegram_user_id INTEGER NOT NULL,
  telegram_chat_id INTEGER NOT NULL,
  user_type VARCHAR(10) NOT NULL,
  diet VARCHAR(10) NOT NULL
);
CREATE TABLE favorites (
  user_id INTEGER,
  canteen_id INTEGER,
  PRIMARY KEY (user_id, canteen_id),
  constraint favorites_user foreign key (user_id) references users (id),
  constraint favorites_canteen foreign key (canteen_id) references canteens (id)
);