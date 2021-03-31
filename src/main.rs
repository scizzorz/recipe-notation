use pest::iterators::Pair;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io::prelude::*;

// you're about to read some awful, hacky, PoC Rust code.
// you've been warned.

#[derive(Parser)]
#[grammar = "recipe.pest"]
struct RecipeParser;

fn main() {
    let mut file = fs::File::open("recipe").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut steps = RecipeParser::parse(Rule::main, &contents).unwrap();
    let mut ing_count = 0;

    println!("digraph {{");
    println!("  rankdir=LR;");

    for step in steps {
        if step.as_rule() == Rule::EOI {
            break;
        }

        let mut inner = step.into_inner();
        let id = inner.next().map(|x| x.as_str().to_string()).unwrap();
        let action = inner.next().map(|x| x.as_str().to_string()).unwrap();
        println!("  step{}[shape=cds, label=\"{}\", style=filled, fillcolor=\"#FFFF99\"];", id, action);

        let ings = inner.next().unwrap().into_inner();
        for ing in ings {
            let label = ing.as_str().to_string();
            let inner = ing.into_inner().next().unwrap();
            match inner.as_rule() {
                Rule::raw_ingredient => {
                    ing_count += 1;
                    println!("  ing{}[shape=box, label=\"{}\", style=filled, fillcolor=\"#9999FF\"];", ing_count, label);
                    println!("  ing{} -> step{}", ing_count, id);
                }
                Rule::prev_ingredient => {
                    println!("  step{} -> step{}", inner.into_inner().as_str().to_string(), id);
                }
                _ => panic!("fuc"),
            }
        }
    }

    println!("}}");
}
