use pest::iterators::Pair;
use std::io::prelude::*;
use std::fs;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "recipe.pest"]
struct RecipeParser;

fn main() {
    let mut file = fs::File::open("recipe").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut main = RecipeParser::parse(Rule::main, &contents);
    dbg!(main);
}
