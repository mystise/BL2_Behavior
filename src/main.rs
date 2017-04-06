extern crate regex;

use std::env;
use std::fs::File;
use std::process;
use std::io::Read;
use std::collections::VecDeque;

use regex::Regex;

fn main() {
    let mut file = None;
    for arg in env::args() {
        let arg = arg.split('=').collect::<Vec<&str>>();
        match arg[0] {
            "-f" => { // File
                file = Some(File::open(arg[1]).unwrap());
            },
            _ => {
            }
        }
    }
    if let None = file {
        process::exit(0);
    }
    let mut file = file.unwrap();
    let mut val = String::new();
    file.read_to_string(&mut val).unwrap();
    let collection_regex = Regex::new(r"\((?:.*?)EventData2=\((.*?)\),BehaviorData=,BehaviorData2=\((.*?)\),VariableData=\((.*?)\),ConsolidatedOutputLinkData=\((.*?)\),ConsolidatedVariableLinkData=\((.*?)\),ConsolidatedLinkedVariables=\((.*?)\)\)").unwrap();
    let collection = collection_regex.captures(&val).unwrap();

    let event_regex = Regex::new(r"\(UserData=\(EventName=(.*?),(?:.*?)\),OutputVariables=\(ArrayIndexAndLength=(.*?)\),OutputLinks=\(ArrayIndexAndLength=(.*?)\)\)").unwrap();
    let mut events = Vec::new();

    for event in event_regex.captures_iter(&collection[1]) {
        events.push((event[1].to_string(), event[2].parse::<i64>().unwrap(), event[3].parse::<i64>().unwrap()));
    }

    let behavior_regex = Regex::new(r"\(Behavior=(?:.*?)([[:word:]]*?)',LinkedVariables=\(ArrayIndexAndLength=(.*?)\),OutputLinks=\(ArrayIndexAndLength=(.*?)\)\)").unwrap();
    let mut behaviors = Vec::new();

    for behavior in behavior_regex.captures_iter(&collection[2]) {
        behaviors.push((behavior[1].to_string(), behavior[2].parse::<i64>().unwrap(), behavior[3].parse::<i64>().unwrap()));
    }

    let variable_regex = Regex::new(r"\(Name=(.*?),Type=(.*?)\)").unwrap();
    let mut variables = Vec::new();

    for variable in variable_regex.captures_iter(&collection[3]) {
        variables.push((variable[1].to_string(), variable[2].to_string()));
    }

    let output_link_regex = Regex::new(r"\(LinkIdAndLinkedBehavior=(.*?),ActivateDelay=(.*?)\)").unwrap();
    let mut output_links = Vec::new();

    for output_link in output_link_regex.captures_iter(&collection[4]) {
        output_links.push((output_link[1].parse::<i64>().unwrap(), output_link[2].parse::<f32>().unwrap()));
    }

    let variable_link_regex = Regex::new(r"\(PropertyName=(.*?),VariableLinkType=(.*?),ConnectionIndex=(.*?),LinkedVariables=\(ArrayIndexAndLength=(.*?)\),CachedProperty=(?:.*?)\)").unwrap();
    let mut variable_links = Vec::new();

    for variable_link in variable_link_regex.captures_iter(&collection[5]) {
        variable_links.push((variable_link[1].to_string(), variable_link[2].to_string(), variable_link[3].parse::<i64>().unwrap(), variable_link[4].parse::<i64>().unwrap()));
    }

    let var_cons_link_regex = Regex::new(r"([[:digit:]]+)").unwrap();
    let mut var_cons_links = Vec::new();

    for var_cons_link in var_cons_link_regex.captures_iter(&collection[6]) {
        var_cons_links.push(var_cons_link[1].parse::<usize>().unwrap());
    }

    for event in &events {
        println!("Event: {}", event.0);
        let var_index = ((event.1 & 0xFFFF0000) >> 16) as usize;
        let var_count = (event.1 & 0xFFFF) as usize;
        println!("Variables:");
        for i in var_index..var_index + var_count {
            println!("- {}", variable_links[i].0);
        }
        println!("Behaviors:");

        let link_index = ((event.2 & 0xFFFF0000) >> 16) as usize;
        let link_count = (event.2 & 0xFFFF) as usize;
        let mut link_stack = VecDeque::new();
        for i in link_index..link_index + link_count {
            link_stack.push_back((&output_links[i], 1, i));
        }
        let mut index_stack = VecDeque::new(); // Prevents infinite loops
        loop {
            let link = link_stack.pop_front();
            if let None = link {
                break;
            }
            let link = link.unwrap();
            let link_index = link.2;
            index_stack.push_back(link_index);
            let depth = link.1;
            let link = link.0;

            for _ in 0..depth {
                print!("  ");
            }

            let behav_index = (link.0 & 0xFFFF) as usize;
            let behav = &behaviors[behav_index];
            println!("{} Delay {} Index {}:{} Link {}", behav.0, link.1, link_index, behav_index, link.0);
            let var_index = ((behav.1 & 0xFFFF0000) >> 16) as usize;
            let var_count = (behav.1 & 0xFFFF) as usize;
            for i in var_index..var_index + var_count {
                for _ in 0..depth {
                    print!("  ");
                }
                let var_link = &variable_links[i];
                println!("- {} : {} : i {}", var_link.0, var_link.1, i);
                let var_index = ((var_link.3 & 0xFFFF0000) >> 16) as usize;
                let var_count = (var_link.3 & 0xFFFF) as usize;
                for i in var_index..var_index + var_count {
                    for _ in 0..depth+1 {
                        print!("  ");
                    }
                    let var = &variables[var_cons_links[i]];
                    println!("- {} : {} : i {}", var.0, var.1, var_cons_links[i]);
                }
            }

            let link_index = ((behav.2 & 0xFFFF0000) >> 16) as usize;
            let link_count = (behav.2 & 0xFFFF) as usize;
            for i in link_index..link_index + link_count {
                if index_stack.contains(&i) {
                    for _ in 0..depth+1 {
                        print!("  ");
                    }
                    println!("LOOP DETECTED {}", i);
                    continue;
                }
                link_stack.push_front((&output_links[i], depth + 1, i));
            }
            if !link_stack.is_empty() {
                for _ in link_stack.front().unwrap().1..depth+1 {
                    index_stack.pop_back();
                }
            }
        }
    }
}
