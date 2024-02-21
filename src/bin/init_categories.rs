use diesel_demo::{connect::establish_connection, repository::category::create_category};

fn main() {
    let connection = &mut establish_connection();

    const NAME: &str = "Cat1";
    const DISP_NAME: &str = "Category 1";
    const NUM_ENTRIES: i32 = 10;
    const PARENT_EID: i32 = 1;

    if let Some(parent) = create_category(connection, NAME, DISP_NAME, 0, PARENT_EID, None) {
        for i in 1..=NUM_ENTRIES {
            let entry_name = format!("{} -- e{}", NAME, i);
            let entry_disp_name = format!("{} -- e{}", DISP_NAME, i);

            let _entry1 = create_category(
                connection,
                &entry_name,
                &entry_disp_name,
                i,
                i,
                Some(parent.id),
            );
        }
    } else {
        println!("Parent category {}, already exists", NAME);
    }
}
