use crate::Hora;
use crate::Dia;

#[derive(Debug)]
pub struct Horario {
    pub dia: Dia, 
    pub inicio: Hora,
    pub fin: Hora
}

impl Horario {
    // Lun: 4:20 - 15:20
    pub fn new(frase: &str) -> Option<Horario> {
        let dia_intervalo = match frase.split_once(':') {
            None => return None,
            Some(c) => c,
        };

        let intervalo = match dia_intervalo.1.split_once('-') {
            None => return None,
            Some(c) => c,
        };

        let dia = Dia::set(dia_intervalo.0.trim());

        let inicio = match Hora::from_str(intervalo.0.trim()){
            None => return None,
            Some(h) => h,
        };
        
        let fin = match Hora::from_str(intervalo.1.trim()){
            None => return None,
            Some(h) => h,
        };

        let sentencia = Horario{dia, inicio, fin};
        return Some(sentencia);
    }
    
}
