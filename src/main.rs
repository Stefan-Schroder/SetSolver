use std::fs::File;
use std::io::{BufReader, BufRead};

use clap::Parser;

use std::char;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

static POSSIBLE_OPTIONS : &'static [i32] = &[
    0b100001010,
    0b100010001,
    0b010001100,
    0b010100001,
    0b001010100,
    0b001100010,
    0b100100100,
    0b010010010,
    0b001001001
];


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

    fn find_forks<'a>(&'a self, fork_list: &mut Vec<[&'a Node; 3]>) {
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
            fork_list.push (
                [
                    self.children[0].as_ref().expect("we checked"),
                    self.children[1].as_ref().expect("we checked"),
                    self.children[2].as_ref().expect("we checked")
                ]
            );
        }
    }

    fn test_forks<'a>(&'a self, fork_list: &mut Vec<[&'a Node; 3]>) {
        let current_option = match fork_list.pop() {
            Some(o) => o,
            None => return,
        };

        if current_option[0].is_end() { // we have hit the end and this is a solution
            for (i, card) in current_option.iter().enumerate() {
                println!("{}. {:?}", i, card.index.as_ref().expect("Logic error in printing options"));
            }
            println!();
        }
        else {
            // Naive solution (with binary so maybe smart, lol)
            // convert the parents and children into matrix containing children
            //                       c1    c2    c3
            let mut node_matrix = [ None, None, None,  // parent 1
                                    None, None, None,  // parent 2
                                    None, None, None]; // parent 3
            let mut matrix_bin = 0b000_000_000;

            for (i, parent) in current_option.iter().enumerate() {
                for (j, child) in parent.children.iter().enumerate() {
                    match child {
                        Some(c) => {
                            node_matrix[i*3+j] = Some(c);
                            matrix_bin |= 1<<(i*3+j);
                        },
                        None => {
                        },
                    }
                }
            }
            for option in POSSIBLE_OPTIONS.iter() {
                if matrix_bin & option == *option {
                    //println!("found {:b} as an option", option);
                    let mut indexes = Vec::new();
                    for i in 0..9 {
                        if option >> i & 1 == 1{
                            indexes.push(i);
                        }
                    }

                    fork_list.push (
                        [
                            node_matrix[indexes[0]].expect("node matrix and matrix bin out of sync with numbers"),
                            node_matrix[indexes[1]].expect("node matrix and matrix bin out of sync with numbers"),
                            node_matrix[indexes[2]].expect("node matrix and matrix bin out of sync with numbers")
                        ]
                    );
                }
            }
        }

        self.test_forks(fork_list);
    }

    fn solve(&self) {
        let mut fork_list = Vec::<[& Node; 3]>::new();
        self.find_forks(&mut fork_list);
        self.test_forks(&mut fork_list);
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

    //println!("flowchart LR");
    //head.mermaid_print(&mut "x".to_string());

    head.solve();

    Ok(())
}
