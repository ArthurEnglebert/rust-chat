-- Your SQL goes here
ALTER TABLE messages
    ADD COLUMN client VARCHAR(255) NOT NULL DEFAULT '<unknown>',
    ADD COLUMN date DATETIME NOT NULL DEFAULT NOW();