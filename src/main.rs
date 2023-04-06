use chrono::{Local, DateTime};
use std::io::Write;
use std::path::Path;
use std::fs::File;
type Throwable = Result<(), Box<dyn std::error::Error>>;

fn main() -> Throwable{
    let row = create_row(Local::now(), "問題");
    write_file(Path::new("hello.txt"), &row)
}
fn create_row(posted: DateTime<Local>, content: &str) -> String{
    format!(
        "[{}] :: {}", 
        posted.format("%Y-%m-%d %H:%M:%S").to_string(),
        content
    )
}

fn write_file(path:&Path, data:&str) -> Throwable {
    let mut file = File::create(&path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}
