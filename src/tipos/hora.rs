use std::ops::Add;

#[derive(Debug)]
pub struct Hora {
    pub hora: u8,
    pub minuto: u8,
    pub hash: u16
}

impl Hora {
    pub fn from_str(marca: &str) -> Option<Hora> {
        let componentes = match marca.split_once(":") {
            None => return None,
            Some((v, h)) => (v, h),
        };
        
        let hora :u8 = match componentes.0.parse() {
            Err(_) => return None,
            Ok(v) => v
        };
        
        let minuto :u8 = match componentes.1.parse(){
            Err(_) => return None,
            Ok(v) => v
        };
        
        let hash = (hora as u16 * 60).add(minuto as u16);
        Some(Hora { hora, minuto, hash})
    }

    // TODO: Validar que no se pase de 24 hora
    fn from_int(marca :u16) -> Hora {
        let hora = (marca / 60) as u8;
        let minuto = (marca / 60) as u8;
       
        Hora { hora, minuto, hash: marca }
    }

}

impl PartialEq for Hora {
    fn eq(&self, other: &Self) -> bool {
       self.hash == other.hash 
    }
}

impl Eq for Hora {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crear_marca_temporal_de_cadena() {
        let ts = Hora::from_str("2:34");
        assert_eq!(ts, Some(Hora{hora: 2, minuto: 34, hash: 154}));
    }

    #[test]
    fn test_crear_marca_temporal_de_int(){
        let ts = Hora::from_int(450);
        assert_eq!(ts, Hora{hora: 7, minuto: 30, hash: 450});
    }

}