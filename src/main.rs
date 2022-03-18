extern crate pest;
#[macro_use]
extern crate pest_derive;


use serde_json::{Map, Value};

use pest::Parser;
use pest::iterators::Pair;
use pest::iterators::Pairs;

use std::fs;

#[derive(Parser)]
#[grammar = "vcl.pest"]
pub struct VCLParser;

use clap::Parser as ClapParser;
#[derive(ClapParser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    let input = fs::read_to_string(args.file).expect("Something went wrong reading the file");
    // println!("{}", input);

    let pairs = VCLParser::parse(Rule::top_level_exp, &input).unwrap_or_else(|e| panic!("{}", e));

    let mut items: Vec<Value> = Vec::new();

    for pair in pairs.into_iter() {
        items.push(parse_pair(pair));
        // A pair is a combination of the rule which matched and a span of input
        
    }

    println!("{}", serde_json::to_string_pretty(&items).unwrap());
}

fn parse_pair(pair: Pair<Rule>) -> Value {
    
    // println!("Span:    {:?}", pair.as_span());

    let rule = pair.as_rule();
    match rule {
        Rule::sub => {
            let mut map: Map<String, Value> = Map::new();
            let mut children = pair.into_inner();
            let name = children.next().unwrap().as_str();

            map.insert("type".to_string(), Value::String("sub".to_string()));
            map.insert("name".to_string(), Value::String(name.to_string()));
            
            if let Some(statements) = children.into_iter().next(){
                map.insert("statements".to_string(), parse_pair(statements));
            }else{
                map.insert("statements".to_string(), Value::Array(Vec::new()));
            }

            return Value::Object(map);
        }
        Rule::ident => {
            let mut map: Map<String, Value> = Map::new();

            map.insert("type".to_string(), Value::String("ident".to_string()));
            map.insert("value".to_string(), Value::String(pair.as_str().to_string()));

            return Value::Object(map);
        }
        Rule::litteral_string => {
            let mut map: Map<String, Value> = Map::new();

            map.insert("type".to_string(), Value::String("string".to_string()));
            map.insert("value".to_string(), Value::String(pair.as_str().to_string()));

            return Value::Object(map);
        }
        Rule::number => {
            let mut map: Map<String, Value> = Map::new();

            map.insert("type".to_string(), Value::String("number".to_string()));
            map.insert("value".to_string(), Value::String(pair.as_str().to_string()));

            return Value::Object(map);
        }
        Rule::set_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("set_exp".to_string()));

            map.insert("assignee".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("assign_operator".to_string(), parse_pair(children.next().unwrap()));
            map.insert("value".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        Rule::unset_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("unset_exp".to_string()));
            map.insert("assignee".to_string(), Value::String(children.next().unwrap().as_str().to_string()));

            return Value::Object(map);
        }
        Rule::assign_operator => {
            return Value::String(pair.as_str().to_string());
        }
        Rule::var_type => {
            return Value::String(pair.as_str().to_string());
        }
        Rule::pragma_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("pragma_exp".to_string()));

            map.insert("pragma_type".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("pragma_name".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("value".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        Rule::weird_top_level_keywords => {
            let mut map: Map<String, Value> = Map::new();

            map.insert("type".to_string(), Value::String("weird_top_level_keywords".to_string()));
            map.insert("value".to_string(), Value::String(pair.as_str().to_string()));

            return Value::Object(map);
        }
        Rule::return_exp => {
            let mut map: Map<String, Value> = Map::new();
            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("return_exp".to_string()));
            map.insert("next".to_string(), Value::String(children.next().unwrap().as_str().to_string()));

            return Value::Object(map);
        }
        Rule::declare_exp => {
            let mut map: Map<String, Value> = Map::new();
            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("declare_exp".to_string()));
            map.insert("name".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("var_type".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        // Remove groupings
        Rule::top_level_exp => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule::expression => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule::expressions => {
            let child_count = pair.clone().into_inner().count();

            if child_count == 1 {
                return parse_pair(pair.into_inner().next().unwrap());
            }

            return parse_pairs(pair.into_inner());
        }
        Rule::value => {
            let child_count = pair.clone().into_inner().count();

            if child_count == 1 {
                return parse_pair(pair.into_inner().next().unwrap());
            }

            return parse_pairs(pair.into_inner());
        }
        Rule::single_value => {
            let child_count = pair.clone().into_inner().count();

            if child_count == 1 {
                return parse_pair(pair.into_inner().next().unwrap());
            }

            return parse_pairs(pair.into_inner());
        }
        Rule::negated_value => {
            let mut map: Map<String, Value> = Map::new();

            map.insert("type".to_string(), Value::String("negated_value".to_string()));
            map.insert("value".to_string(), parse_pairs(pair.into_inner()));

            return Value::Object(map);
        }
        Rule::logic_exp => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule::backend_map_object => {
            return parse_pairs(pair.into_inner());
        }
        Rule::enclosed_value => {
            let child_count = pair.clone().into_inner().count();

            if child_count == 1 {
                return parse_pair(pair.into_inner().next().unwrap());
            }

            return parse_pairs(pair.into_inner());
        }
        Rule::indent => {
            let child_count = pair.clone().into_inner().count();

            if child_count == 1 {
                return parse_pair(pair.into_inner().next().unwrap());
            }

            return parse_pairs(pair.into_inner());
        }
        Rule::rtime_type => {
            return Value::String(pair.as_str().to_string());
        }
        Rule::rtime_value => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("rtime_value".to_string()));

            map.insert("value".to_string(), parse_pair(children.next().unwrap()));
            map.insert("period".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            

            return Value::Object(map);
        }
        Rule::backend_option => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("backend_option".to_string()));

            map.insert("key".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("value".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        Rule::backend => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("backend".to_string()));

            map.insert("name".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("options".to_string(), parse_pairs(children));

            return Value::Object(map);
        }
        Rule::restart_exp => {
            let mut map: Map<String, Value> = Map::new();
            map.insert("type".to_string(), Value::String("restart_exp".to_string()));
            return Value::Object(map);
        }
        Rule::if_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("if_exp".to_string()));

            map.insert("logic_exp".to_string(), parse_pair(children.next().unwrap()));
            map.insert("match_statements".to_string(), parse_pairs(children));

            return Value::Object(map);
        }
        Rule::if_else_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("if_exp".to_string()));

            map.insert("logic_exp".to_string(), parse_pair(children.next().unwrap()));

            let first = children.next().unwrap();

            match first.as_rule() {
                Rule::if_else_seperator => {
                    map.insert("match_statements".to_string(), Value::Array(Vec::new()));
                    
                    let child = children.next();
                    if child.is_some() { 
                        map.insert("miss_statements".to_string(), parse_pairs(child.unwrap().into_inner()));
                    }else{
                        map.insert("miss_statements".to_string(), Value::Array(Vec::new()));
                    }
                }
                _ => {
                    map.insert("match_statements".to_string(), parse_pairs(first.into_inner()));
                    let _ = children.next();

                    let child = children.next();
                    if child.is_some() { 
                        map.insert("miss_statements".to_string(), parse_pairs(child.unwrap().into_inner()));
                    }else{
                        map.insert("miss_statements".to_string(), Value::Array(Vec::new()));
                    }
                }
            }

            return Value::Object(map);
        }
        Rule::comp_operator => {
            return Value::String(pair.as_str().to_string());
        }
        Rule::compare_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("compare_exp".to_string()));

            map.insert("value_one".to_string(), parse_pair(children.next().unwrap()));
            map.insert("comp_operator".to_string(), parse_pair(children.next().unwrap()));
            map.insert("value_two".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        Rule::error_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("error_exp".to_string()));
            map.insert("code".to_string(), parse_pair(children.next().unwrap()));
            map.insert("message".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        Rule::synthetic_exp => {
            let mut map: Map<String, Value> = Map::new();

            let children = pair.into_inner();

            map.insert("type".to_string(), Value::String("synthetic_exp".to_string()));

            let mut content: String = String::new();

            for child in children {
                content.push_str(child.as_str());
            }

            map.insert("value".to_string(), Value::String(content));

            return Value::Object(map);
        }
        Rule::func_call => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("func_call".to_string()));

            map.insert("name".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            map.insert("args".to_string(), parse_pairs(children.next().unwrap().into_inner()));

            return Value::Object(map);
        }
        Rule::inline_if_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("inline_if_exp".to_string()));

            map.insert("logic_exp".to_string(), Value::String(children.next().unwrap().as_str().to_string()));
            
            let _ = children.next(); // Take comma

            let mut first_args: Vec<Value> = Vec::default();
            
            while let Some(child) = children.next() {
                match child.as_rule() {
                    Rule::comma => {
                        break;
                    }
                    _ => {
                        first_args.push(parse_pair(child));
                    }
                }
            }

            let second_args = parse_pairs(children);

            map.insert("first_args".to_string(), Value::Array(first_args));
            map.insert("second_args".to_string(), second_args);

            return Value::Object(map);
        }
        Rule::calc_operator => {
            return Value::String(pair.as_str().to_string());
        }
        Rule::calc_exp => {
            let mut map: Map<String, Value> = Map::new();

            let mut children = pair.into_inner();

            map.insert("type".to_string(), Value::String("calc_exp".to_string()));

            map.insert("value_one".to_string(), parse_pair(children.next().unwrap()));
            map.insert("comp_operator".to_string(), parse_pair(children.next().unwrap()));
            map.insert("value_two".to_string(), parse_pair(children.next().unwrap()));

            return Value::Object(map);
        }
        // Print and skip
        _ => {
            eprintln!("Didn't handle type: {:?}", rule);
            return parse_pairs(pair.into_inner());
        }
    }
}

fn parse_pairs(pairs: Pairs<Rule>) -> Value {
    let mut items: Vec<Value> = Vec::new();

    for pair in pairs {
        items.push(parse_pair(pair));
    }

    Value::Array(items)
}