CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    value VARCHAR NOT NULL,
    display_value VARCHAR NOT NULL,
    display_order INTEGER NOT NULL,
    enum_id INTEGER NOT NULL,
    parent_id INTEGER REFERENCES categories(id)
);