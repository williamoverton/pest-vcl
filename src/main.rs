extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "vcl.pest"]
pub struct VCLParser;

fn main() {
    let input = include_str!("input.vcl");
    // println!("{}", input);

    let pairs = VCLParser::parse(Rule::top_level_exp, input).unwrap_or_else(|e| panic!("{}", e));
    
    for pair in pairs {
        print_pair(pair, 0);
        // A pair is a combination of the rule which matched and a span of input
        
    }
}

fn print_pair(pair: Pair<Rule>, depth: u64){
    let padding = (0..depth).map(|_| " ").collect::<String>();
    
    // println!("Span:    {:?}", pair.as_span());

    let rule = pair.as_rule();
    match rule {
        Rule::ident => {
            println!("{}Ident({})", padding, pair.as_str());
        }
        Rule::litteral_string => {
            println!("{}String({})", padding, pair.as_str());
        }
        Rule::number => {
            println!("{}Number({})", padding, pair.as_str());
        }
        Rule::comp_operator => {
            println!("{}CompOperator({})", padding, pair.as_str());
        }
        Rule::calc_operator => {
            println!("{}CalcOperator({})", padding, pair.as_str());
        }
        Rule::assign_operator => {
            println!("{}AssignOperator({})", padding, pair.as_str());
        }
        // Rule::set_exp => {
            
        //     let mut pairs = pair.into_inner();
        //     let ident = pairs.next().unwrap().as_str();
        //     let operator = pairs.next().unwrap().as_str();
        //     let value = pairs.next().unwrap().as_str();
        //     println!("{}SetExpression({} {} {})", padding, ident, operator, value);
        //     // for inner_pair in pair.into_inner() {
        //     //     println!("{} = {}", 
        //     //         inner_pair.as_rule().to_string(),
        //     //         inner_pair.as_str(),
        //     //     );
        //     //     print_pair(inner_pair, depth + 2);
        //     // }
        // }
        _ => {
            println!("{}{:?}", padding, rule);
            for inner_pair in pair.into_inner() {
                print_pair(inner_pair, depth + 1);
            }
        }
    }

    
}