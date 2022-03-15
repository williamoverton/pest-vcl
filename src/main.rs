extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "vcl.pest"]
pub struct VCLParser;

fn main() {
    let input = include_str!("input.vcl");
    // println!("{}", input);

    let pairs = VCLParser::parse(Rule::top_level_exp, input).unwrap_or_else(|e| panic!("{}", e));
    
    for pair in pairs {
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());

        for inner_pair in pair.into_inner() {
            println!("Inner:   {:?}", inner_pair.as_rule());
            println!("InnerSpan:   {:?}", inner_pair.as_span());
        }
    }
}
