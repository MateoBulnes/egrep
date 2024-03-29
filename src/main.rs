use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use egrep::regex::Regex;


fn leer_archivo(path: &String) -> Result<Vec<String>, String> {
    match File::open("src/".to_string() + &path) {
        Ok(file) => {
            let reader = io::BufReader::new(file);

            let mut frutas = Vec::new();

            // Itera sobre cada línea del archivo
            for line in reader.lines() {
                let line = line.map_err(|e| format!("Error al leer el archivo: {}", e))?;
                frutas.push(line);
            }

            Ok(frutas)
        }
        Err(_) => Err("error abriendo el archivo".to_string()),
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let path_archivo = &args[2];
    let expresion = &args[1];

    //let expresion = Regex{};

    match leer_archivo(path_archivo) {
        Ok(leidos) => {
            
            
        }
        Err(e) => println!("Error: {}", e),
    }
}
