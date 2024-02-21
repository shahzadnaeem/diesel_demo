use crate::models::{Category, NewCategory};
use diesel::prelude::*;
use diesel::PgConnection;

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
        .filter(value.eq(category_name))
        .count()
        .get_result::<i64>(conn)
        .expect("Error finding Category Name");

    n == 1
}
