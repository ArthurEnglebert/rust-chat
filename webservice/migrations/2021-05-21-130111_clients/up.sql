-- Your SQL goes here
CREATE TABLE clients(
    uuid VARCHAR(255) NOT NULL,

    name VARCHAR(255) NOT NULL,
    pass TEXT NOT NULL,

    PRIMARY KEY(uuid),
    UNIQUE KEY(name)
)