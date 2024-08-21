use std::fs::File;
use std::io::{BufReader, BufRead};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,
}

#[derive(Debug)]
struct Node {
    variant: u8,
    index: i32,
    children: Vec<Node>,
}

impl Node {
    fn new(v: u8)->Self {
        Node {
            variant: v,
            index: -1,
            children: Vec::new(),
        }
    }

    fn set_index(&mut self, i: i32) {
        self.index = i;
    }

    fn get_index(&self)->i32 {
        self.index
    }

    fn is_end(& self)->bool {
        self.index != -1
    }

    fn insert_and_return(&mut self, v: u8)-> &mut Node {
        println!("\ninside {} looking for {}", self.variant, v);
        let mut return_index = 0;
        let mut found = false;

        for (i, c) in self.children.iter_mut().enumerate() {
            if c.variant == v {
                return_index = i;
                found = true;
                break;
            }
        }

        if !found {
            println!("variant not found adding");
            let mut n = Node::new(v);
            self.children.push(n);
            return_index = self.children.len() - 1;
        }
        else
        {
            println!("variant found!");
        }

        return &mut self.children[return_index];
    }

    fn mermaid_print(&self, s: &mut String) {
        if self.is_end() {
            s.push((self.variant + '0' as u8) as char);
            println!("{} --> i{}([{}])", s, self.index, translate(s)); 
            s.pop();
        }
        else {
            for (i, c) in self.children.iter().enumerate() {
                println!("{}{} --> {}{}{}[{}]", s, self.variant, s, self.variant, c.variant, c.variant);

                s.push((self.variant + '0' as u8) as char);
                c.mermaid_print(s);
                s.pop();
            }
        }
    }

}

fn translate(s: &String)->String{
    let colour = ["red", "green", "purple"];
    let shape = ["bean", "pill", "diamond"];
    let shade = ["clear", "shaded", "solid"];
    let number = ["one", "two", "three"];

    let mut ret = String::new();
    ret.push_str(number[s.chars().nth(1).unwrap() as usize - '0' as usize - 1]);
    ret.push_str(" ");
    ret.push_str(colour[s.chars().nth(2).unwrap() as usize - '0' as usize - 1]);
    ret.push_str(" ");
    ret.push_str(shade[s.chars().nth(3).unwrap() as usize - '0' as usize - 1]);
    ret.push_str(" ");
    ret.push_str(shape[s.chars().nth(4).unwrap() as usize - '0' as usize - 1]);
    
    if s.chars().nth(4).unwrap() as usize - '0' as usize > 1 {
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
        current_node.set_index(i as i32);
    }

    head.mermaid_print(&mut "".to_string());

    Ok(())
}
