use std::fmt::Display;
use std::ops::Add;

#[derive(Debug)]
pub struct Hora {
    pub hora: u8,
    pub minuto: u8,
    pub hash: u16
}

impl Hora {
    pub fn from_parser(hora: &str, minuto: &str) -> Hora {
        // Como viene de un parser, tenemos que asumir que los valores están bien
        let hora: u8 = hora.parse().unwrap();
        let minuto: u8 = minuto.parse().unwrap();
        
        let hash = (hora as u16 * 60).add(minuto as u16);
        Hora { hora, minuto, hash}
    }

}

impl PartialEq for Hora {
    fn eq(&self, other: &Self) -> bool {
       self.hash == other.hash 
    }
}

impl Eq for Hora {}

impl Display for Hora {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.hora, self.minuto)
    }
}

impl PartialOrd for Hora {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
       self.hash.partial_cmp(&other.hash) 
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_crear_de_parser(){
        let ts = Hora::from_parser("08", "23");
        assert_eq!(ts, Hora{hora: 8, minuto: 23, hash: 503});
    }

    #[test]
    fn test_comparacion(){
        let menor = Hora::from_parser("08", "30");
        let mayor = Hora::from_parser("09", "30");
        assert!(menor < mayor);
    }
}