pub mod lingua;
pub mod whichlang;
mod record;

use std::error::Error;
use std::time::*;

use crate::whichlang::*;
use crate::lingua::*;

fn main() -> Result<(), Box<dyn Error>>{
    let mut file = csv::Reader::from_path("data/train.csv")?;
    let mut result;
    let mut start;
    let mut duration ;

    start = std::time::Instant::now();
    result = test_lingua(&mut file)?;
    duration = start.elapsed().as_millis();

    print!("duration: {}us\n", duration); 
    print!("lingua: result {}/{} | {}%\n", result.0, result.1, result.0 as f32 / result.1 as f32);

    file = csv::Reader::from_path("data/train.csv")?;
    start = std::time::Instant::now();
    result = test_whichlang(&mut file)?;
    duration = start.elapsed().as_millis();

    print!("duration: {}us\n", duration); 
    print!("whichlang: result {}/{} | {}%\n", result.0, result.1, result.0 as f32 / result.1 as f32);
    Ok(())
}
