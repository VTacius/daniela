use std::fmt::Display;

use super::Hora;
use super::Dia;

#[derive(Debug)]
pub struct Horario {
    pub dia: Dia, 
    pub inicio: Hora,
    pub fin: Hora
}

impl Horario {
    // Lun: 4:20 - 15:20
    pub fn new(dia: Dia, inicio: Hora, fin: Hora) -> Horario {
        Horario{dia, inicio, fin}
    }
    
}

impl PartialEq for Horario {
    fn eq(&self, other: &Self) -> bool {
        self.dia.value().actual == other.dia.value().actual 
            && self.inicio == other.inicio
            && self.fin == other.fin
    }
}

impl Eq for Horario {}

impl Display for Horario {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {} - {}", self.dia, self.inicio, self.fin)
    }  
}