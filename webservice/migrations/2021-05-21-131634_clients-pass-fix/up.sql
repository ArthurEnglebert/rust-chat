-- Your SQL goes here
ALTER TABLE clients
    ADD COLUMN salt TEXT NOT NULL;