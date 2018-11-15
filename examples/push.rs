extern crate dockworker;
extern crate hyper;

use dockworker::{Docker, models::UserPassword};

fn main() {
    let mut docker = Docker::from_env().unwrap();

    let (name, tag) = ("alpine", "latest");
    docker
        .create_image(name, tag)
        .unwrap()
        .for_each(|_| print!("."));

    let serveraddress = "localhost:5000";
    docker.set_credential(UserPassword::new(
        "someusername",
        "somepassword",
        "someusername@example.com",
        serveraddress,
    ));

    println!("pulled: {}:{}", name, tag);
    docker
        .push_image(&format!("{}/{}", serveraddress, name), tag)
        .unwrap();
    println!("pushed: {}:{}", name, tag);
}
