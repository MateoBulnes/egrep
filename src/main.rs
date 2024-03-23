use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn leer_archivo(path: &String) -> Result<Vec<String>, String> {
    //let  archivo: File;

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

fn filtrar(expresion: &String, leidos: Vec<String>) -> Vec<String> {
    leidos.into_iter() // Convertimos el vector en un iterador para poder aplicar los métodos de filtrado
        .filter(|s| s.contains(expresion)) // Filtramos las cadenas que contienen la expresión
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path_archivo = &args[2];
    let expresion = &args[1];

    match leer_archivo(path_archivo) {
        Ok(leidos) => {
            let resultados = filtrar(expresion, leidos);

            println!("Resultados: ");
            for r in resultados {
                println!("{}", r);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}
