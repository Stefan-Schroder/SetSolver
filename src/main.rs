use std::fs::File;
use std::io::{BufReader, BufRead};

use std::collections::VecDeque;

use clap::Parser;

use std::char;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
struct Node {
    index: Option<String>,
    children: [Option<Box<Node>>; 3],
    debug_name: String,
}

impl Node {
    fn new(s : String)->Self {
        Node {
            index: None,
            children: [None, None, None],
            debug_name: s,
        }
    }

    fn set_index(&mut self, i: String) {
        self.index = Some(i);
    }

    fn get_index(&self)->& String {
        self.index.as_ref().expect("You need to check first you hoe")
    }

    fn is_end(& self)->bool {
        self.index.is_some()
    }

    fn insert_and_return(&mut self, v: u8, debug_name: String)-> &mut Node {
        if self.children[v as usize].is_none() {
            self.children[v as usize] = Some(Box::new(Node::new(debug_name)));
        }

        return self.children[v as usize].as_mut().unwrap();
    }

    fn mermaid_print(&self, s: &mut String) {
        if self.is_end() {
            println!("{} --> i{}({:?})", s, s, self.index.as_ref().expect("Bish i checked this"));
        } else {
            for (i, c) in self.children.iter().enumerate() {
                if c.is_none() {
                    continue;
                }
                println!("{} --> {}{}[{}]", s, s, i, i + 1);
                s.push((i as u8 + b'0') as char);
                c.as_ref().expect("If this crashes i am dumb").mermaid_print(s);
                s.pop();
            }
        }
    }

    fn find_forks<'a>(&'a self, fork_list: &mut VecDeque<[&'a Node; 3]>) {
        let mut at_end = false;
        let mut has_all = true;
        for child in self.children.as_ref() {
            match child {
                Some(c) => {
                    if !at_end {
                        if c.is_end() {
                            at_end = true;
                        } else {
                            c.find_forks(fork_list);
                        }
                    }
                },
                None => {
                    has_all = false;
                },
            }
        }

        if has_all {
            fork_list.push_back (
                [
                    self.children[0].as_ref().expect("we checked"),
                    self.children[1].as_ref().expect("we checked"),
                    self.children[2].as_ref().expect("we checked")
                ]
            );
        }
    }

    fn test_forks<'a>(&'a self, fork_list: &mut VecDeque<[&'a Node; 3]>) {
        if fork_list.len() == 0 {
            return;
        }

        let mut current_option = &mut fork_list.front().expect("Our len check failed");

        if current_option[0].is_end() { // we have hit the end and this is a solution
            for (i, card) in current_option.iter().enumerate() {
                println!("{}. {:?}", i, card.index.as_ref().expect("Logic error in printing options"));
            }
            fork_list.pop_front();
            return; // I think this should be here
        }

        // find options

    }

    fn solve(&self) {
        let mut fork_list = VecDeque::<[& Node; 3]>::new();
        self.find_forks(&mut fork_list);

        // Debug print fork
        /*
        for (i, fork) in fork_list.iter().enumerate() {
            println!("Fork {}\n{}\n{}\n{}\n", i,
                fork.first.debug_name,
                fork.second.debug_name,
                fork.third.debug_name);

        }
        */

        /*
        let mut space = [0, 0, 0];

        for parent in parents.iter() {
            for (i, child) in parent.children.iter().enumerate() {
                if child.is_none() {
                    continue;
                }
                space[i] += 1;
            }
        }

        // the 4th one is for a * space
        let mut explore = [false,false,false,true];

        for (i,x) in space.iter().enumerate() {
            if space[i] == parents.len() { // all parents have this in their space
                explore[i] = true;
            }
            else {
                explore[3] = false; // one space is not present so the all space is not possible
            }
        }

        // I want to explore * first
        for (i,x) in explore.iter().enumerate().rev() {
            if !x { // only explore where we have options
                continue;
            }
            if i == 3 {
                continue;
            }
            if parents.children[i].expect("logic error should not explore here").is_end() {
                println!("{:?}", parents.children[i].expect().index());
            }
        }

        println!("{:?}", space);
        println!("{:?}", explore);
        */

    }

}

fn translate(s: &String)->String{
    let colour = ["red", "green", "purple"];
    let shape = ["bean", "pill", "diamond"];
    let shade = ["clear", "shaded", "solid"];
    let number = ["one", "two", "three"];

    let mut ret = String::new();
    ret.push_str(number[s.chars().nth(0).unwrap() as usize - '0' as usize]);
    ret.push_str(" ");
    ret.push_str(colour[s.chars().nth(1).unwrap() as usize - '0' as usize]);
    ret.push_str(" ");
    ret.push_str(shade[s.chars().nth(2).unwrap() as usize - '0' as usize]);
    ret.push_str(" ");
    ret.push_str(shape[s.chars().nth(3).unwrap() as usize - '0' as usize]);
    
    if s.chars().nth(3).unwrap() as usize - '0' as usize > 0 {
        ret.push_str("s");
    } else { 
        ret.push_str("");
    }

    return ret;
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let file = BufReader::new(File::open(args.input)?);

    let mut head = Node::new("x".to_string());

    for (i, result) in file.lines().enumerate() {
        let line = match result {
            Ok(line) => line,
            Err(e) => panic!("Could not read line {}: {}", i, e),
        };

        let mut current_node: &mut Node = &mut head;
        let mut current_debug_name = String::new();
        for c in line.chars() {
            current_debug_name.push((c as u8 + 1) as char);
            current_node = current_node.insert_and_return(c as u8 - '0' as u8, current_debug_name.clone());
        }
        current_node.set_index(translate(&line));
    }

    println!("flowchart LR");
    head.mermaid_print(&mut "x".to_string());

    println!("\n============\nSolving...");
    head.solve();

    Ok(())
}
