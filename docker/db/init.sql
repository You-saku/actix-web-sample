CREATE TABLE IF NOT EXISTS users(
    id SERIAL NOT NULL PRIMARY KEY , -- SERIAL はオートインクリメント
    name VARCHAR (50) NOT NULL,
    email VARCHAR (50) default '',
    age INTEGER default 0,
    created_at date NOT NULL, -- date は日付
    updated_at date NOT NULL,
    deleted_at date
);

INSERT INTO users VALUES (1, 'you-saku', 'sample@example.com', 0, '2023-01-01 00:00:00', '2023-01-01 00:00:00', null);
