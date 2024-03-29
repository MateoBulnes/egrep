use std::collections::VecDeque;

use crate::{regexRepeticiones::RegexRepeticiones, regexValor::RegexValor};


struct RegexPaso {
    valor: RegexValor,
    repeticiones: RegexRepeticiones
}

pub struct Regex {
    pasos: Vec<RegexPaso>
}

impl Regex{
    pub fn new(expresion: &str) -> Result<Self, &str>{
        let mut pasos: Vec<RegexPaso> = Vec::new();

        let mut caracteres_iter = expresion.chars();
        while let Some(c) = caracteres_iter.next() {
            let paso = match c {
                '.' => Some(RegexPaso {
                    valor: RegexValor::Comodin,
                    repeticiones: RegexRepeticiones::Exacto(1)
                }),
                'a'..='z' => Some(RegexPaso{
                    valor: RegexValor::Caracter(c),
                    repeticiones: RegexRepeticiones::Exacto(1)
                }),
                '*' => {
                    if let Some(last) = pasos.last_mut() {
                        last.repeticiones = RegexRepeticiones::Simple;
                    } else{
                        return Err("Se encontro un caracter '*' no soportado");
                    }

                    None
                },
                '\\' => match caracteres_iter.next(){
                    Some(literal) => Some(RegexPaso{
                        valor: RegexValor::Caracter(literal),
                        repeticiones: RegexRepeticiones::Exacto(1)
                    }),
                    None => return Err("Error de caracter '\\' no soportado")
                }
                _ => return Err("Se encontro un caracter no soportado")
            };

            if let Some(p) = paso{
                pasos.push(p);
            }
            
        }

        Ok(Regex{ pasos })
    }

    pub fn validar_expresion(self, valor: &str) -> Result<bool, &str> {
        if !valor.is_ascii() {
            return Err("El input no es ascii");
        }

        let mut fila = VecDeque::from(self.pasos);
        let mut index: usize = 0;

        while let Some(paso) = fila.pop_front() {
            match paso.repeticiones {
                RegexRepeticiones::Exacto(n) =>{
                    for _ in [1, n] {
                        let size = paso.valor.matches(&valor[index..]);

                        if size == 0{
                            return Ok(false);
                        }

                        index += size;
                    }
                },
                RegexRepeticiones::Simple => {
                    let mut matcheando = true;

                    while matcheando {
                        let size = paso.valor.matches(&valor[index..]);

                        if size != 0{
                            index += size;
                        } else{
                            matcheando = false;
                        }
                    }
                },
                RegexRepeticiones::Rango { min, max } => todo!(),
            }
        }

        Ok(true)
    } 
}


