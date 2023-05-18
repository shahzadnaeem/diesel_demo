use diesel::prelude::*;
use diesel_demo::*;

use self::models::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("{}\n", "=".repeat(post.title.len()));
        println!("{}", post.body);
    }

    let unpublished = posts
        .filter(published.eq(false))
        .count()
        .get_result::<i64>(connection)
        .expect("Error loading unpublished posts count");

    println!("There are {} unpublished posts", unpublished);
}
