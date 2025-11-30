use std::num::ParseFloatError;
use std::env;

use geoconv::est97_to_wgs84;
use geoconv::wgs84_to_est97;



fn main() {
    let args: Vec<String> = env::args().collect();
    let control:Result<(f64, f64), String> = match &args.len() {
        4 => {
            match args[1].to_uppercase().trim() {
                "-W" => match parse(&args[2], &args[3]) {
                    Ok(ver) => match wgs84_to_est97(&ver.0, &ver.1) {
                        Ok(c) => Ok(c),
                        Err(e) => Err(e.to_string())
                    },
                    Err(e) => {
                        help();
                        Err(e.to_string())
                    }
                },
                _ => {
                    help();
                    Err(String::from("Wrong arguments."))
                }

            }
         },
        3 => match parse(&args[1], &args[2]) {
                    Ok(ver) => match est97_to_wgs84(&ver.0, &ver.1) {
                        Ok(c) => Ok(c),
                        Err(e) => Err(e.to_string())
                    },
                    Err(e) => {
                        help();
                        Err(e.to_string())
                    }
        },
        _ =>  {
            help();
            Err(String::from("Arguments not understood."))
        }
    };

    println!("{:?}", &control);
}


fn parse(from_x: &String, from_y: &String) -> Result<(f64,f64), ParseFloatError> {
    Ok((from_x.parse::<f64>()?, from_y.parse::<f64>()?))
}


fn help() {
    println!("
Convert between EST97 and WGS 84 geographical coordinates.
USAGE: 
  geoconv [OPTIONS] <from_x> <from_y>

OPTIONS:
  -w, From WGS84 to EST97. If omitted then reverse.
");
}


//TODO: implement some nice argparser library,
//TODO: implement all other identities too.