use diesel_demo::core::prelude::*;

fn pop_generic(when: bool) -> Result<i32> {
    if when {
        Err(Error::Generic("pop".to_string()))
    } else {
        Ok(123)
    }
}

fn pop_static(when: bool) -> Result<i32> {
    if when {
        Err(Error::Static("pop"))
    } else {
        Ok(123)
    }
}

fn pop_connect(when: bool) -> Result<i32> {
    let res = connect(when)?;

    Ok(res)
}

fn connect(when: bool) -> Result<i32> {
    if when {
        Err(Error::DieselConnection(
            diesel::result::ConnectionError::InvalidConnectionUrl("pooostgres://".to_string()),
        ))
    } else {
        Ok(007)
    }
}

fn main() -> Result<()> {
    println!("Let's errorize!");

    pop_generic(false)?;
    // pop_generic(true)?;
    pop_static(false)?;
    // pop_static(true)?;
    pop_connect(false)?;
    let res = pop_connect(true);
    if let Err(err) = &res {
        println!("pop_connect: {} - {:?}", err, err);
    }
    res?;

    Ok(())
}
