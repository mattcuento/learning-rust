use std::cmp::PartialEq;
use std::fmt::Display;
use std::io;
use std::io::ErrorKind;
use std::str::FromStr;

#[derive(Debug)]
#[derive(PartialEq)]
enum TempUnit {
   Celsius,
   Fahrenheit,
}

impl FromStr for TempUnit {
   type Err = io::Error;
   fn from_str(s: &str) -> Result<Self, Self::Err> {
      match s.trim().to_lowercase().as_str() {
         "c" | "celsius" => Ok(TempUnit::Celsius),
         "f" | "fahrenheit" => Ok(TempUnit::Fahrenheit),
         _ => Err(io::Error::new(ErrorKind::InvalidInput, "invalid unit")),
      }
   }
}

impl Display for TempUnit {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let str = match self {
         TempUnit::Fahrenheit => "F".to_string(),
         TempUnit::Celsius => "C".to_string(),
      };
      write!(f, "{}", str)
   }
}

fn main() {
   let mut raw_input = String::new();

   println!("Enter temperature value:");

   io::stdin()
       .read_line(&mut raw_input)
       .expect("Failed to read line");

   let input_temp: f64 = raw_input.trim().parse().expect("Not a number");
   raw_input.clear();

   println!("Enter temperature unit:");

   io::stdin()
       .read_line(&mut raw_input)
       .expect("Failed to read line");

   let input_unit: TempUnit = raw_input.trim().parse().expect("Not a number");

   let converted_temp = match input_unit {
      TempUnit::Celsius => { 9.0/5.0 * (input_temp - 32.0) },
      TempUnit::Fahrenheit => { 5.0/9.0 * (input_temp - 32.0) },
   };

   println!("{}{} is {}{}", input_temp, input_unit, converted_temp, if input_unit == TempUnit::Celsius { "F" } else { "C" } );
}
