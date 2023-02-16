use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("argc={}",args.len());
    for i in 0..args.len(){
        println!("argv=[{}]={}",i,args[i]);
    }
}
