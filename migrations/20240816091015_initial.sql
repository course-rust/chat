-- Add migration script here
-- create user table
CREATE TABLE IF NOT EXISTS users(
    id SERIAL PRIMARY KEY,
    fullname VARCHAR(64) NOT NULL,
    email VARCHAR(64) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);
-- create index on users for email
CREATE UNIQUE INDEX IF NOT EXISTS idx_email ON users(email);

-- create chat type: single, group, private_channel, public_channel
CREATE TYPE chat_type AS ENUM ('single', 'group', 'private_channel', 'public_channel');

-- create chat table
CREATE TABLE IF NOT EXISTS chats(
    id SERIAL PRIMARY KEY,
    name VARCHAR(128) NOT NULL UNIQUE,
    type chat_type NOT NULL,
    members BIGINT[] NOT NULL,
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP
);

-- create message table
CREATE TABLE IF NOT EXISTS messages(
    id SERIAL PRIMARY KEY,
    chat_id BIGINT NOT NULL,
    sender_id BIGINT NOT NULL,
    content TEXT NOT NULL,
    images TEXT[],
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (chat_id) REFERENCES chats(id),
    FOREIGN KEY (sender_id) REFERENCES users(id)
);
-- create index on messages for chat_id and created_at order by created_at desc
CREATE INDEX IF NOT EXISTS idx_chat_id_created_at ON messages(chat_id, created_at desc);

-- create index on messages for sender_id
CREATE INDEX idx_sender_id_created_at ON messages(sender_id, created_at desc);
