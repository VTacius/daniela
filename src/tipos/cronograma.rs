use std::collections::HashMap;

//use pgx::StringInfo;
//use pgx::JsonInOutFuncs;
//use pgx::PostgresType;
//use pgx::WithTypeIds;
//use pgx::pg_extern;

use pgx::*;
use serde::{Deserialize, Serialize};

use super::Tick;
use super::Horario;

#[derive(Debug, Deserialize, Serialize, PostgresType)]
pub struct Cronograma {
    pub sentencias: Vec<String>,
    pub contenido: HashMap<u8, Vec<Tick>>,
}

impl Cronograma {
    pub fn new(sentencias: Vec<Horario>) -> Cronograma{
        // Lo inicializo ahora en cero, pero acá posiblemente estaré dentro de un ciclo
        let mut contenido: HashMap<u8, Vec<Tick>> = HashMap::new();
        
        for sentencia in &sentencias {
            // Por acá iniciaría el loop
            let dia_actual = sentencia.dia.value().actual;
            let dia_siguiente = sentencia.dia.value().siguiente;
            if sentencia.inicio.hash < sentencia.fin.hash {
                let elemento = Tick::new(sentencia.inicio.hash, sentencia.fin.hash);
                contenido = Cronograma::agregar_elemento(dia_actual, contenido, elemento)
            } else {
                let elemento = Tick::new(sentencia.inicio.hash, 1440);
                contenido = Cronograma::agregar_elemento(dia_actual, contenido, elemento);
                let elemento = Tick::new(0, sentencia.fin.hash);
                contenido = Cronograma::agregar_elemento(dia_siguiente, contenido, elemento)

            };

        }
        
        // La convertirmos a string
        let sentencias = sentencias.iter().map(|s| s.to_string()).collect();
        Cronograma{ contenido, sentencias }
    }

    pub fn agregar_elemento(dia: u8, contenido: HashMap<u8, Vec<Tick>>, elemento: Tick) -> HashMap<u8, Vec<Tick>>{
        let mut contenido = contenido.clone();
        match contenido.get_mut(&dia) {
            None => {
                contenido.insert(dia, vec![elemento]);
            },
            Some(t) => {
                let valores = Cronograma::normalizar_elementos(&t, elemento);
                *t = valores;
            } 
        }

        contenido
    }
    pub fn normalizar_elementos(lista: &Vec<Tick>, buscado: Tick) -> Vec<Tick> {
        let mut lista = lista.clone();
        if let Some(indice) = lista.iter().position(|e|{e.solapa(buscado)}){
            lista[indice] = lista[indice].merge(buscado);
        } else {
            lista.push(buscado)
        }

        lista.to_vec()
    } 

}
