use crate::AddArgs;

pub fn add_command(args: &AddArgs) -> Result<(), String> {
    println!("\t{:?}", args);

    if args.name == "pop" {
        return Err("Pop!".to_string());
    }

    let mut full_args = args.clone();

    if full_args.disp_name == None {
        full_args.disp_name = Some(format!("Cat {}", full_args.name).to_string());
    }

    println!("\t{:?}", full_args);

    Ok(())
}
