use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What's yo title?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove that pesky training newline

    println!(
        "\nGet some writin in! {} (Press {} when finished)",
        body, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);

    println!("Saved draft {} with id {}", title, post.id)
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
