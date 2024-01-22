-- Test library greetings
CREATE TABLE IF NOT EXISTS greetings (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    stamp DATE NOT NULL
);

-- Actually useful stuff

CREATE TABLE IF NOT EXISTS genders (
    gender TEXT PRIMARY KEY NOT NULL
);

INSERT INTO genders VALUES ( 'woman' );
INSERT INTO genders VALUES ( 'man' );
INSERT INTO genders VALUES ( 'nonbinary' );

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    full_name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL,
    dob DATE NOT NULL,
    gender TEXT NOT NULL,

    FOREIGN KEY(gender) REFERENCES genders(gender)
);

CREATE TABLE IF NOT EXISTS wants_to_date (
    user_id INTEGER NOT NULL,
    gender TEXT NOT NULL,

    PRIMARY KEY (user_id, gender), -- The pair target, who is unique
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS additional_data (
    user_id INTEGER PRIMARY KEY NOT NULL,
    json TEXT NOT NULL,            -- additional JSON which will not be accessed often

    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS chats (
    id INTEGER PRIMARY KEY AUTOINCREMENT
);

CREATE TABLE IF NOT EXISTS messages (
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    stamp DATETIME NOT NULL,
    text TEXT NOT NULL,

    PRIMARY KEY(stamp, user_id),
    FOREIGN KEY(chat_id) REFERENCES chats(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE IF NOT EXISTS participants (
    chat_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    
    PRIMARY KEY(chat_id, user_id),
    FOREIGN KEY(chat_id) REFERENCES chats(id),
    FOREIGN KEY(user_id) REFERENCES users(id)
);
