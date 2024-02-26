CREATE TABLE cats (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    alt_name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE catents (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    alt_name VARCHAR NOT NULL,
    enum_id INTEGER NOT NULL,
    cat_id INTEGER NOT NULL REFERENCES cats(id) ON DELETE RESTRICT,
    UNIQUE(cat_id, name),
    UNIQUE(cat_id, alt_name),
    UNIQUE(cat_id, enum_id)
);