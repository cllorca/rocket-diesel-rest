use self::models::*;
use diesel::prelude::*;
use hello::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        // .filter(published.eq(true))
        .limit(5)
        .load::<User>(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("-----------\n");
        println!("{}", user.username);
    }
}