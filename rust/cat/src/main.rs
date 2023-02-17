use std::env;
use std::fs::File;
use std::fs::OpenOptions;
fn main() {
    
    let args: Vec<String> = env::args().collect();
    println!("argc={}",args.len());
    for i in 0..args.len(){
        println!("argv=[{}]={}",i,args[i]);
    }
}

fn do_cat(path: String){
    let buffer_size:usize = 2048;
    let mut buf:Vec<char> = vec!['0';usize];
    
}