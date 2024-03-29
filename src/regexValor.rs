use crate::regexClase::RegexClase;

pub enum RegexValor {
    Caracter(char),
    Comodin,
    Clase(RegexClase)
}

impl RegexValor {
    pub fn matches(&self, valor: &str) -> usize{
        match self {
            RegexValor::Caracter(l) => {
                if valor.chars().next() == Some(*l){
                    l.len_utf8()
                } else{
                    0
                }
            },
            RegexValor::Comodin => {
                if let Some(c) = valor.chars().next() {
                    c.len_utf8()
                } else {
                    0
                }
            },
            _=> 0
        }
    }
}