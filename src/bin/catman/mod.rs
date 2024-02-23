use diesel_demo::{
    connect::establish_connection,
    repository::category::{
        category_by_name, create_category, next_parent_enum_id_for, num_category_entries,
    },
};

use crate::AddArgs;

pub fn add_command(args: &AddArgs) -> Result<(), String> {
    // println!("\t{:?}", args);

    let mut full_args = args.clone();

    if full_args.disp_name == None {
        full_args.disp_name = Some(format!("Cat {}", full_args.name).to_string());
    }

    add_category(
        &full_args.name,
        &full_args.disp_name.unwrap(),
        full_args.num_entries,
    )
}

pub fn add_category(name: &str, disp_name: &str, num_entries: i32) -> Result<(), String> {
    let connection = &mut establish_connection();

    if let Some(next_enum_id) = next_parent_enum_id_for(connection, name) {
        println!(
            "# Creating category: {}/{}, Enum ID: {} with {} entries",
            name, disp_name, next_enum_id, num_entries
        );

        if let Some(parent) =
            create_category(connection, name, disp_name, 0, next_enum_id as i32, None)
        {
            for i in 1..=num_entries {
                let entry_name = format!("e{}", i);
                let entry_disp_name = format!("Entry {}.e{}", name, i);

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
                println!("#   Category created: {:?}", cat);
            } else {
                return Err(format!("Failed to create category '{}'", name));
            }

            if let Some(entries_created) = num_category_entries(connection, name) {
                println!(
                    "#   {} category entries created for category '{}'",
                    entries_created, name
                );
            }
        } else {
            return Err(format!(
                "Unable to create category '{}', now exists, but didn't when we checked!",
                name
            ));
        }
    } else {
        return Err(format!("Category '{}' already exists", name));
    }

    Ok(())
}
