-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
    id INTEGER NOT NULL PRIMARY KEY , -- SERIAL はオートインクリメント
    name VARCHAR (50) NOT NULL,
    email VARCHAR (50) default '',
    age INTEGER default 0,
    created_at date NOT NULL, -- date は日付
    updated_at date NOT NULL,
    deleted_at date
);

