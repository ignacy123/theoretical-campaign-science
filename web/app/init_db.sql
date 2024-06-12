CREATE TABLE IF NOT EXISTS demo (
    key varchar(32) NOT NULL PRIMARY KEY,
    value integer NOT NULL
);

INSERT INTO demo (key, value)
VALUES
    ('Jeden', 1),
    ('Dwa', 2),
    ('Cztery', 4),
    ('Osiem', 8)
ON CONFLICT DO NOTHING;



CREATE TABLE IF NOT EXISTS sales (
    username varchar(64) NOT NULL PRIMARY KEY CHECK (LENGTH(username) >= 4)
);



CREATE TABLE IF NOT EXISTS clicks (
    username varchar(64) NOT NULL CHECK (LENGTH(username) >= 4),
    ad varchar(32) NOT NULL CHECK (LENGTH(ad) >= 1),
    PRIMARY KEY (username, ad)
);



CREATE TABLE IF NOT EXISTS posts (
    id serial PRIMARY KEY,
    file_path varchar(64) NOT NULL,
    title varchar(1024) NOT NULL
);
