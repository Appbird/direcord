use chrono::Local;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::env;

fn main(){
    println!("{}", time_output());
    let args = env::args();
}

fn time_output() -> String{
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}
#[allow(unused)]
fn write_file()-> Result<(), Box<dyn std::error::Error>>{
    let path = Path::new("hello.txt");
    let mut file = File::create(&path)?;
    let sentence = "タスク";
    file.write_all(sentence.as_bytes())?;
    
    Ok(())
}
