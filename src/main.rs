use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io::prelude::*;

#[derive(Parser)]
#[grammar = "recipe.pest"]
struct RecipeParser;

fn main() {
    let mut file = fs::File::open("recipe").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut steps = RecipeParser::parse(Rule::main, &contents).unwrap();
    for step in steps {
        if step.as_rule() == Rule::EOI {
            break;
        }

        let mut inner = step.into_inner();
        let id = inner.next().map(|x| x.as_str().to_string()).unwrap();
        let action = inner.next().map(|x| x.as_str().to_string()).unwrap();
        println!("{} {}", id, action);

        let ings = inner.next().unwrap().into_inner();
        for ing in ings {
            let label = ing.as_str().to_string();
            let inner = ing.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::raw_ingredient => {
                    dbg!(&inner);
                }
                Rule::prev_ingredient => {
                    println!("=> #{}", inner.into_inner().as_str().to_string());
                }
                _ => panic!("fuc"),
            }
        }
    }
}
