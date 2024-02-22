use diesel_demo::{
    connect::establish_connection,
    repository::category::{
        all_categories, category_by_name, category_entries, create_category,
        next_parent_enum_id_for, num_category_entries,
    },
};

use std::env::args;

fn main() {
    const NAME: &str = "Cat1";
    const DISP_NAME: &str = "Category 1";
    const NUM_ENTRIES: i32 = 10;

    let connection = &mut establish_connection();

    let args = args().into_iter().skip(1).collect::<Vec<_>>();

    let mut name = NAME;
    let mut disp_name = DISP_NAME;
    let mut num_entries = NUM_ENTRIES;

    if args.len() > 0 {
        name = &args[0];
        disp_name = &args[1];
        num_entries = args[2].parse::<i32>().unwrap();
    }

    if let Some(next_enum_id) = next_parent_enum_id_for(connection, name) {
        println!(
            "Creating category: {}/{} Enum ID: {} with {} entries",
            name, disp_name, next_enum_id, num_entries
        );

        if let Some(parent) =
            create_category(connection, name, disp_name, 0, next_enum_id as i32, None)
        {
            for i in 1..=num_entries {
                let entry_name = format!("{} -- e{}", name, i);
                let entry_disp_name = format!("{} -- e{}", disp_name, i);

                let _entry1 = create_category(
                    connection,
                    &entry_name,
                    &entry_disp_name,
                    i,
                    i,
                    Some(parent.id),
                );
            }

            if let Some(cat) = category_by_name(connection, name) {
                println!("Category created: {:?}", cat);
            } else {
                println!("ERR!: Category not created: {}", name);
            }

            if let Some(entries_created) = num_category_entries(connection, name) {
                println!(
                    "{} category entries created for category: {}",
                    entries_created, name
                );
            } else {
                println!("No category entries found! - category not created");
            }
        } else {
            println!("Unable to create category: already exists!");
        }
    } else {
        println!("Category {}, already exists", name);
    }

    // All categories...
    let categories = all_categories(connection);

    println!("\nAll Categories");
    for cat in categories {
        println!("\t{:?}", cat);
        if let Some(entries) = category_entries(connection, &cat.value) {
            for entry in entries {
                println!("\t\t{:?}", entry);
            }
        }
    }
}
