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

#[derive(Debug)]
struct Node {
    index: Option<String>,
    children: [Option<Box<Node>>; 3],
}

impl Node {
    fn new(v: u8)->Self {
        Node {
            index: None,
            children: [None, None, None],
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

    fn insert_and_return(&mut self, v: u8)-> &mut Node {
        match &self.children[v as usize]{
            Some(_s) => {
            },
            None => {
                self.children[v as usize] = Some(Box::new(Node::new(v)));
            },
        };

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

    //fn solve(&self, parents: Vec<& Node>) {
    //    let mut space = [0, 0, 0];
    //
    //    for parent in parents.iter() {
    //        for child in parent.children.iter() {
    //            space[child.variant as usize - 1] += 1;
    //        }
    //    }
    //
    //    // the 4th one is for a * space
    //    let mut explore = [false,false,false,true];
    //
    //    for (i,x) in space.iter().enumerate() {
    //        if space[i] == parents.len() { // all parents have this in their space
    //            explore[i] = true;
    //        }
    //        else {
    //            explore[3] = false; // one space is not present so the all space is not possible
    //        }
    //    }
    //
    //    for (i,x) in explore.iter().enumerate().rev() {
    //    }
    //
    //    println!("{:?}", space);
    //    println!("{:?}", explore);
    //
    //}

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

    let mut head = Node::new(0);

    for (i, result) in file.lines().enumerate() {
        let line = match result {
            Ok(line) => line,
            Err(e) => panic!("Could not read line {}: {}", i, e),
        };

        let mut current_node: &mut Node = &mut head;
        for c in line.chars() {
            current_node = current_node.insert_and_return(c as u8 - '0' as u8);
        }
        current_node.set_index(translate(&line));
    }

    head.mermaid_print(&mut "x".to_string());

    //println!("Solving...");
    //let root_vec: Vec<& Node> = vec![& head];
    //head.solve(root_vec);

    Ok(())
}
