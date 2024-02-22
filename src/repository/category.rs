use crate::models::{Category, Catenumid, NewCategory};
use diesel::PgConnection;
use diesel::{prelude::*, sql_query};

pub fn create_category(
    conn: &mut PgConnection,
    value: &str,
    display_value: &str,
    display_order: i32,
    enum_id: i32,
    parent_id: Option<i32>,
) -> Option<Category> {
    use crate::schema::categories;

    if !category_exists(conn, value) {
        let new_category = NewCategory {
            value,
            display_value,
            display_order,
            enum_id,
            parent_id,
        };

        Some(
            diesel::insert_into(categories::table)
                .values(&new_category)
                .get_result(conn)
                .expect("Error saving new category"),
        )
    } else {
        None
    }
}

pub fn category_exists(conn: &mut PgConnection, category_name: &str) -> bool {
    use crate::schema::categories::dsl::*;

    let n = categories
        .filter(value.eq(category_name).and(parent_id.is_null()))
        .count()
        .get_result::<i64>(conn)
        .expect("Error finding Category Name");

    n > 0
}

pub fn all_categories(conn: &mut PgConnection) -> Vec<Category> {
    use crate::schema::categories::dsl::*;

    let cats: Vec<Category> = categories
        .filter(parent_id.is_null())
        .load(conn)
        .expect("Error finding Category Name");

    cats
}

pub fn category_by_name(conn: &mut PgConnection, category_name: &str) -> Option<Category> {
    use crate::schema::categories::dsl::*;

    let mut cats: Vec<Category> = categories
        .filter(value.eq(category_name).and(parent_id.is_null()))
        .load(conn)
        .expect("Error finding Category Name");

    if cats.len() == 1 {
        Some(cats.swap_remove(0))
    } else {
        None
    }
}

pub fn category_id_by_name(conn: &mut PgConnection, category_name: &str) -> Option<i32> {
    if let Some(cat) = category_by_name(conn, category_name) {
        Some(cat.id)
    } else {
        None
    }
}

pub fn num_category_entries(conn: &mut PgConnection, category_name: &str) -> Option<i32> {
    use crate::schema::categories::dsl::*;

    if let Some(cat_parent_id) = category_id_by_name(conn, category_name) {
        let n = categories
            .filter(parent_id.eq(cat_parent_id))
            .count()
            .get_result::<i64>(conn)
            .expect("Error getting Num Category Entries");

        Some(n as i32)
    } else {
        None
    }
}

// Use direct SQL and load into model that holds the expected return value

pub fn next_parent_enum_id(conn: &mut PgConnection) -> i64 {
    let next = sql_query("SELECT nextval('catenumid') as enum_id")
        .load::<Catenumid>(conn)
        .unwrap();

    next[0].enum_id
}

pub fn next_parent_enum_id_for(conn: &mut PgConnection, category_name: &str) -> Option<i64> {
    if !category_exists(conn, category_name) {
        Some(next_parent_enum_id(conn))
    } else {
        None
    }
}
