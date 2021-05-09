use std::env;

mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
//mod cli;
// mod nQueen;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    if command == "print" {
        print::run();
    } else if command == "vars" {
        vars::run();
    } else if command == "types" {
        types::run();
    } else if command == "strings" {
        strings::run();
    } else if command == "tuples" {
        tuples::run();
    } else if command == "arrays" {
        arrays::run();
    } else if command == "vectors" {
        vectors::run();
    } else if command == "conditionals" {
        conditionals::run();
    } else if command == "loops" {
        loops::run();
    } else if command == "functions" {
        functions::run();
    } else if command == "pointer_ref" {
        pointer_ref::run();
    } else if command == "structs" {
        structs::run();
    } else if command == "enums" {
        enums::run();
    } 
    // else if command == "nQueen" {
    //     nQueen::run();
    // }
    //cli::run();
}
