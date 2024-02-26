use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel_demo::models::CatEnt;
use diesel_demo::models::NewCatEnt;
use diesel_demo::{
    connect::establish_connection,
    models::{Cat, NewCat},
};

use anyhow::Context;

use crate::{AppResult, TestArgs};

pub fn test_command(args: &TestArgs) -> AppResult {
    let mut full_args = args.clone();

    if full_args.alt_name == None {
        full_args.alt_name = Some(format!("Category {}", full_args.name).to_string());
    }

    if full_args.num_entries >= 0 {
        create_category(&full_args)?
    } else {
        delete_category(&full_args)?
    }

    Ok(())
}

fn create_category(args: &TestArgs) -> AppResult {
    use diesel_demo::schema::{catents, cats};

    let connection = &mut establish_connection();

    // Create
    println!(
        "Creating category: {} with {} entries ...",
        args.name, args.num_entries
    );

    let new_cat = NewCat {
        name: &args.name,
        alt_name: &(args.alt_name.as_ref().unwrap()),
    };

    // TODO: Sort out AppResult and ensure all possible errors can be converted
    let cat: Cat = diesel::insert_into(cats::table)
        .values(&new_cat)
        .get_result(connection)
        .context(format!(
            "Insert failed for Cat: with name '{}/{}'",
            new_cat.name, new_cat.alt_name
        ))?;

    println!("New cat created: {:?}", cat);

    for i in 1..=args.num_entries {
        let name = format!("e{}", i);
        let alt_name = format!("{} e{}", cat.name, i);

        let new_catent = NewCatEnt {
            name: &name,
            alt_name: &alt_name,
            enum_id: i as i32,
            cat_id: cat.id,
        };

        let catent: CatEnt = diesel::insert_into(catents::table)
            .values(&new_catent)
            .get_result(connection)?;

        println!("  New catent created: {:?}", catent);
    }

    Ok(())
}

fn delete_category(args: &TestArgs) -> AppResult {
    use diesel_demo::schema::catents::dsl as catents_dsl;
    use diesel_demo::schema::cats::dsl as cats_dsl;

    let connection = &mut establish_connection();

    // Delete
    println!("Deleting category: {} and all entries ...", args.name);

    let cat: Option<Cat> = cats_dsl::cats
        .filter(cats_dsl::name.eq(&args.name))
        .first(connection)
        .optional()?;

    if let Some(cat) = cat {
        // Something to delete...
        let n = diesel::delete(catents_dsl::catents.filter(catents_dsl::cat_id.eq(cat.id)))
            .execute(connection)?;

        println!(" {} catents deleted", n);

        let n =
            diesel::delete(cats_dsl::cats.filter(cats_dsl::id.eq(cat.id))).execute(connection)?;

        println!(" {} cat deleted", n);
    } else {
        println!(" Nothing to do");
    }

    Ok(())
}
