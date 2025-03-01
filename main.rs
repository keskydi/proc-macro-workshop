use std::marker::PhantomData;

// Write code here.
//
// To see what the code looks like after macro expansion:
//     $ cargo expand
//
// To run the code:
//     $ cargo run
use derive_builder::Builder;
use derive_debug::CustomDebug;

#[derive(CustomDebug)]
pub struct Field {
    name: &'static str,
    bitmask: u8,
    test:bool
}

#[derive(Builder)]
pub struct Command {
    executable: String,
    #[builder(each = "arg")]
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());
//
    //let command = Command::builder()
    //    .executable("cargo".to_owned())
    //    .args(vec!["build".to_owned(), "--release".to_owned()])
    //    .env(vec![])
    //    .current_dir("..".to_owned())
    //    .build()
    //    .unwrap();
    //assert!(command.current_dir.is_some());
}
