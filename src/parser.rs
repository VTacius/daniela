use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::tipos::Horario;
use crate::tipos::Hora;
use crate::tipos::Dia;

#[derive(Parser)]
#[grammar = "cronograma.pest"]
struct CronogramaParser {

}

pub fn parsear_linea(sentencia: &str) -> Horario {
    //let contenido = "Lun: 7:30 - 8:40";
    let mut parseo = match CronogramaParser::parse(Rule::HORARIO, sentencia){
        Err(e) => {
            panic!("{}", e.to_string());
        },
        Ok(v) => {
            v
        }
    };

    // Estamos suponiendo que la estructura de datos es correcta, y que por eso podemos usar tantos unwrap
    let pr = parseo.next().unwrap();
    let mut contenido = pr.into_inner();
    
    let dia = crear_dia(contenido.next().unwrap());
    let inicio = crear_hora(contenido.next().unwrap());
    let fin = crear_hora(contenido.next().unwrap());
   
    Horario::new(dia, inicio, fin)
}

fn crear_dia(pr: Pair<Rule>) -> Dia {
    let mut contenido = pr.into_inner();
    let dia = contenido.next().unwrap();
    match dia.as_rule() {
        Rule::DOM => Dia::Domingo, 
        Rule::LUN => Dia::Lunes, 
        Rule::MAR => Dia::Martes, 
        Rule::MIE => Dia::Miercoles, 
        Rule::JUE => Dia::Jueves, 
        Rule::VIE => Dia::Viernes, 
        Rule::SAB => Dia::Sabado,
        
        _ => todo!(), 
    }
}

fn crear_hora(pr: Pair<Rule>) -> Hora {
    let mut contenido = pr.into_inner();
    let hora = contenido.next().unwrap();
    let minuto = contenido.next().unwrap();

    Hora::from_parser(hora.as_str(), minuto.as_str())
}