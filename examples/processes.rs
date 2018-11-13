extern crate dockworker;

use dockworker::{ContainerListOptions, Docker};

fn main() {
    let docker = Docker::from_env().unwrap();
    let opts = ContainerListOptions::default();
    if let Some(container) = docker.containers(opts).unwrap().get(0) {
        for process in docker.processes(container).unwrap() {
            println!("{:#?}", process);
        }
    }
}
