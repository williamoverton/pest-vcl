extern crate pest;
#[macro_use]
extern crate pest_derive;

use serde_json::{Map, Value};

use pest::Parser;
use pest::iterators::Pair;
use pest::iterators::Pairs;

#[derive(Parser)]
#[grammar = "vcl.pest"]
pub struct VCLParser;

fn main() {
    let input = include_str!("input.vcl");
    // println!("{}", input);

    let pairs = VCLParser::parse(Rule::top_level_exp, input).unwrap_or_else(|e| panic!("{}", e));
    
    let mut items: Vec<Value> = Vec::new();

    for pair in pairs {
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
            map.insert("statements".to_string(), parse_pairs(children.into_iter()));

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
        Rule:: top_level_exp => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule:: expression => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule:: value => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
        }
        Rule:: single_value => {
            let child = pair.into_inner().next().unwrap();
            return parse_pair(child);
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
        Rule::assign_operator => {
            return Value::String(pair.as_str().to_string());
        }
        _ => {
            println!("{:?}", rule);
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