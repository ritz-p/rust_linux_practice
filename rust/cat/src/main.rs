use std::env;
use std::fs::File;
use std::io::{BufReader,BufRead};
fn main() {
    let args: Vec<String> = env::args().collect();
    args[1..].iter().for_each(|path| {
        match File::options().read(true).open(path) {
            Ok(file) => {
                println!("<file={}>",path);
                let mut buf_file = BufReader::new(file);
                do_cat(&mut buf_file);
                println!("</file={}>\n",path);
            }
            Err(e) => println!("{}: {}", path, e),
            }
    })
}

fn do_cat(stream: &mut dyn BufRead) -> (){
    let mut buffer = String::new();
    loop{
        match stream.read_line(&mut buffer){
            Ok(0) => break,
            Ok(_) => {
                print!("{}",buffer);
                buffer.clear();
            }
            Err(e) => {
                println!("{}",e);
                break;
            }
        }
    }
}