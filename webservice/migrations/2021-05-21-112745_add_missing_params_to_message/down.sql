-- This file should undo anything in `up.sql`
ALTER TABLE messages
    DROP COLUMN client,
    DROP COLUMN date;