-- Your SQL goes here
CREATE TABLE IF NOT EXISTS genders (
    gender TEXT PRIMARY KEY NOT NULL
);

INSERT INTO genders VALUES ( 'woman' );
INSERT INTO genders VALUES ( 'man' );
INSERT INTO genders VALUES ( 'nonbinary' );

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    full_name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL,
    dob DATE NOT NULL,
    gender TEXT NOT NULL,
    FOREIGN KEY(gender) REFERENCES genders(gender)
);

CREATE TABLE IF NOT EXISTS wants_to_date (
    id INTEGER NOT NULL,
    gender TEXT NOT NULL,
    PRIMARY KEY (id, gender), -- The pair target, who is unique
    FOREIGN KEY(id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS additional_data (
    id INTEGER PRIMARY KEY NOT NULL,
    json TEXT NOT NULL,            -- additional JSON which will not be accessed often
    FOREIGN KEY(id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS greetings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    stamp DATE NOT NULL
);
