use std::fmt::Display;
use logos::Logos;

#[derive(Debug)]
pub struct Hora {
    pub hash: u16
}

#[derive(Logos, Debug, PartialEq)]
enum Tokens {
    #[regex(r"[0-9][0-9]")]
    Componente,

    #[regex(r"\s?[aA]")]
    AM,

    #[regex(r"\s?[pP]")]
    PM,

    #[error]
    #[regex(r" :", logos::skip)]
    Error,
}
impl Hora {
    pub fn from_lexer(contenido: &str) -> Option<Hora> {
        let mut lexer = Tokens::lexer(contenido);
       
        // Conseguimos las horas
        lexer.next().unwrap();
        // Es que ya sabemos que el formato es el correcto, por eso lo de usar unwrap tan tranquilo
        let hora: u8 = lexer.slice().parse().unwrap();

        // Conseguimos los minutos, recuerda que tienes que pasar por el :
        lexer.next().unwrap();
        lexer.next().unwrap();
        let minuto: u8 = lexer.slice().parse().unwrap();
      
        // Si tenemos un token PM, convertimos a horario militar agregando 12 a todo lo menor a 12
        let sufijo = lexer.next(); 
        let hora = if let Some(Tokens::PM) = sufijo {
            if hora < 12 { hora + 12} else { hora }    
        } else if let Some(Tokens::AM) = sufijo {
            if hora >= 12 { hora - 12 } else { hora }
        }else {
            hora
        };

        // Estamos asumiendo que si alguien puso AM, es porque en realidad quiere que sean en la mañana
       
        let hash = (hora as u16 * 60) + minuto as u16;
        
        Some(Hora{ hash})
    }

}

impl PartialEq for Hora {
    fn eq(&self, other: &Self) -> bool {
       self.hash == other.hash 
    }
}

impl Eq for Hora {}

// TODO: Necesita mostrarse en formato 12
// TODO: El problema con el cero, por favor
impl Display for Hora {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sufijo = if self.hash >= 720 { "PM" } else { "AM" };
        let norma = self.hash / 60;
        let hora = if norma == 0 {
            12
        } else {
            norma 
        };
        let minuto = self.hash - (norma * 60);
        write!(f, "{:>2}:{:0>2} {}", hora, minuto, sufijo)
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
        let ts = Hora::from_lexer("08:21 A.M");
        assert_eq!(ts, Some(Hora{hash: 501}));
    }
    
    #[test]
    fn test_crear_de_parser_caso_practico_01(){
        let ts = Hora::from_lexer("12:24");
        assert_eq!(ts, Some(Hora{hash: 744}));
    }
    
    #[test]
    fn test_crear_de_parser_caso_practico_02(){
        let ts = Hora::from_lexer("15:24");
        assert_eq!(ts, Some(Hora{hash: 924}));
    }
    
    #[test]
    fn test_crear_de_parser_caso_practico_03(){
        let ts = Hora::from_lexer("7:00 PM");
        assert_eq!(ts, Some(Hora{hash: 1140}));
    }
    
    #[test]
    fn test_crear_de_parser_caso_practico_04(){
        let ts = Hora::from_lexer("12:01AM");
        assert_eq!(ts, Some(Hora{hash: 1}));
    }



    #[test]
    fn test_crear_de_parser_tarde(){
        let ts = Hora::from_lexer("08:21 P.M");
        assert_eq!(ts, Some(Hora{hash: 1221}));
    }
    
    #[test]
    fn test_crear_de_parser_tarde_caso_02(){
        let ts = Hora::from_lexer("08:21pm");
        assert_eq!(ts, Some(Hora{hash: 1221}));
    }
    
    #[test]
    fn test_crear_mediodia(){
        let ts = Hora::from_lexer("12:00pm");
        assert_eq!(ts, Some(Hora{hash: 720}));
    }
    
    #[test]
    fn test_crear_mediodia_militar(){
        let ts = Hora::from_lexer("12:00");
        assert_eq!(ts, Some(Hora{hash: 720}));
    }
    
    #[test]
    fn test_crear_medianoche(){
        let ts = Hora::from_lexer("12:00am");
        assert_eq!(ts, Some(Hora{hash: 0}));
    }
    
    #[test]
    fn test_crear_medianoche_militar(){
        let ts = Hora::from_lexer("00:00");
        assert_eq!(ts, Some(Hora{hash: 0}));
    }
    
    #[test]
    fn test_crear_de_parser_hora_militar_tarde(){
        let ts = Hora::from_lexer("20:21");
        assert_eq!(ts, Some(Hora{hash: 1221}));
    }
    
    #[test]
    fn test_crear_de_parser_hora_militar_tarde_caso_02(){
        let ts = Hora::from_lexer("20:21 PM");
        assert_eq!(ts, Some(Hora{hash: 1221}));
    }
    
    #[test]
    fn test_crear_de_parser_hora_militar_tarde_caso_03(){
        // Asumamos que se equivoco y que en realidad quiere que sea en la mañana
        let ts = Hora::from_lexer("20:21 AM");
        assert_eq!(ts, Some(Hora{hash: 501}));
    }
    
    #[test]
    fn test_comparacion(){
        let menor = Hora::from_lexer("08:30").unwrap();
        let mayor = Hora::from_lexer("09:30").unwrap();
        assert!(menor < mayor);
    }
    
    #[test]
    fn test_comparacion_caso_02(){
        let menor = Hora::from_lexer("7:00AM").unwrap();
        let mayor = Hora::from_lexer("7:00 PM").unwrap();
        assert!(menor < mayor);
    }
    
    #[test]
    fn test_cadena(){
        let hora = Hora::from_lexer("7:00AM").unwrap();
        assert_eq!(hora.to_string(), " 7:00 AM".to_owned());
    }
    
    #[test]
    fn test_cadena_mediodia(){
        let hora = Hora::from_lexer("12:24").unwrap();
        assert_eq!(hora.to_string(), "12:24 PM".to_owned());
    }
}