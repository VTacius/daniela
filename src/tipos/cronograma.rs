use std::collections::HashMap;

use pgx::*;
use serde::Deserialize;
use serde::Serialize;

use super::Tick;
use super::Horario;

#[derive(Debug, Deserialize, Serialize, PostgresType)]
pub struct Cronograma {
    pub sentencias: Vec<String>,
    pub horarios: HashMap<u8, Vec<Tick>>,
}

impl Cronograma {
    pub fn new(horarios: Vec<Horario>) -> Cronograma{
        // Convertirmos los sentencias a un formato más legible
        let sentencias = horarios.iter().map(|s| s.to_string()).collect();
        
        // Convertimos los horarios a un formato que debería ser más rápido para calcular
        let horarios = crear_horarios_serializables(horarios); 
        Cronograma{ horarios, sentencias }
    }

}

fn crear_horarios_serializables(horarios: Vec<Horario>) -> HashMap<u8, Vec<Tick>>{
    horarios.iter()
        .map(convertir_horario_to_tick)
        .flatten()
        .fold(HashMap::new(), crear_mapa_chronograma)
}

fn crear_mapa_chronograma(mut acc: HashMap<u8, Vec<Tick>>, item: (u8, Tick)) -> HashMap<u8, Vec<Tick>> {
    match acc.get_mut(&item.0) {
        None => {
            acc.insert(item.0, vec![item.1]);
        },
        Some(v) => {
            // TODO: Falta la normalización en esto
            *v = [v.clone(), vec![item.1]].concat();
        }
    }
    acc
}

fn convertir_horario_to_tick(horario: &Horario) -> Vec<(u8, Tick)>{
    if horario.inicio < horario.fin {
        vec![
            (horario.dia.value().actual, Tick::new(horario.inicio.hash, horario.fin.hash))
        ]
    } else {
        vec![
            (horario.dia.value().actual, Tick::new(horario.inicio.hash, 1440)),
            (horario.dia.value().siguiente, Tick::new(0, horario.fin.hash))
        ]
    }
}

#[derive(Copy, Clone)]
struct Declaracion {
    pub lista: [u8; 3],
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tipos::{Horario, Dia, Hora, Tick};

    #[test]
    fn test_particiona_intervalo(){
        let horario = Horario::new(Dia::Sabado, Hora::from_parser("20", "22"), Hora::from_parser("1", "1"));
        let elemento = convertir_horario_to_tick(&horario);
        assert_eq!(elemento, vec![(7, Tick{inicio: 1222, fin: 1440} ), (1, Tick{inicio: 0, fin: 61})]);
    }
}