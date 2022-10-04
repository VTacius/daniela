
use std::fmt::Display;
use logos::Logos;

use crate::tipos::{Dia, Hora, Horario};


#[derive(Logos, Debug, PartialEq)]
enum Dict {

    #[regex(r"lune?s?")]
    Lunes,
  
    #[regex(r"mart?e?s?")]
    Martes,

    #[regex(r"mi[eé]r?c?o?l?e?s?")]
    Miercoles,

    #[regex(r"juev?e?s?")]
    Jueves,

    #[regex(r"vier?n?e?s?")]
    Viernes,

    #[regex(r"s[aá]ba?d?o?")]
    Sabado,
    
    #[regex(r"domi?n?g?o?")]
    Domingo,
  
    // 01:20 AM, 7:30 AM
    #[regex(r"0?[0-9]:[0-5][0-9](\s?[aApP]\.?[mM]\.?)?")]
    // 11:30 A.M 11:50 p.m
    #[regex(r"1[0-9]:[0-5][0-9](\s?[aApP]\.?[mM]\.?)?")]
    // 23:40
    #[regex(r"2[0-3]:[0-5][0-9](\s?[aApP]\.?[mM]\.?)?")]
    Hora, 

    #[error]
    #[regex(r"[ \t]", logos::skip)]
    Error,
}

pub fn parsear_contenido(contenido: &str) -> Result<Vec<Horario>, UbicacionError> {
    let lineas: Vec<_>= contenido.split(&['\n', ';']).map(|linea| linea.to_lowercase()).collect();
    let mut horarios: Vec<Horario> = vec![];
    
    for (orden, linea) in lineas.iter().enumerate() {
        match procesar_linea(orden,&linea) {
            Ok(horario) => horarios.push(horario) ,
            Err(e) => return Err(e),
        }
    }

    return Ok(horarios);

}

#[derive(Debug, Clone)]
enum TipoError {
    Dia,
    Inicio,
    Fin, 
    Terminal
}

#[derive(Debug, Clone)]
pub struct UbicacionError {
    tipo: TipoError,
    linea: usize,
    caracter: usize
}

impl UbicacionError {
    fn new(tipo: TipoError, linea: usize, caracter: usize) -> UbicacionError {
        UbicacionError{tipo, linea, caracter}
    }
}

impl Display for UbicacionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error en linea {:} caracter {:}", self.linea, self.caracter) 
    }
}

fn procesar_linea(orden: usize, linea: &str) -> Result<Horario, UbicacionError>{
    let  mut lex = Dict::lexer(linea);
   
    // Primero, sacamos el día y lo convertimos a nuestra conocido enum
    let dia = match lex.next() {
        Some(Dict::Lunes) => Dia::Lunes,
        Some(Dict::Martes) => Dia::Martes,
        Some(Dict::Miercoles) => Dia::Miercoles,
        Some(Dict::Jueves) => Dia::Jueves,
        Some(Dict::Viernes) => Dia::Viernes,
        Some(Dict::Sabado) => Dia::Sabado,
        Some(Dict::Domingo) => Dia::Domingo,
        _ => return Err(UbicacionError::new(TipoError::Dia, orden, lex.span().start))
    };
   
    // TODO: ¿Podés reutilizar un poco acá
    // Sacamos la hora inicial. Primero aseguramos que haya un siguiente token...
    let inicio = match lex.next() {
        Some(Dict::Hora)  => Hora::from_lexer(lex.slice()),
        _ => return Err(UbicacionError::new(TipoError::Inicio, orden, lex.span().start))
    };
    
    // ...Y luego que el formato sea adecuado
    let inicio = match inicio {
        Some(hora) => hora,
        None => return Err(UbicacionError::new(TipoError::Inicio, orden, lex.span().start))
    };
    
   
    // De nuevo, ahora con hora final. Nos aseguramos de que haya un siguiente token...
    let fin = match lex.next() {
        Some(Dict::Hora)  => Hora::from_lexer(lex.slice()),
        _ => return Err(UbicacionError::new(TipoError::Fin, orden, lex.span().start))
    };

    // ... Y luego, que el formato sea el adecuado
    let fin = match fin {
        Some(hora) => hora,
        None => return Err(UbicacionError::new(TipoError::Fin, orden, lex.span().start))
    };

    // Nos aseguramos que este haya sido el final del viaje
    if let Some(_) = lex.next() {
        return Err(UbicacionError::new(TipoError::Terminal, orden, lex.span().start))
    } 

    Ok(Horario::new(dia, inicio, fin))
}