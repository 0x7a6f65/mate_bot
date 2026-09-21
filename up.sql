CREATE TABLE mates (
    id BIGINT NOT NULL PRIMARY KEY,
    display_name TEXT NOT NULL,
    count INTEGER NOT NULL
);

CREATE TABLE allowed_chats (
    id BIGINT NOT NULL PRIMARY KEY
);
