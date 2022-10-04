use std::fmt::Display;

#[derive(Debug)]
pub struct Day {
    pub actual: u8,
    pub siguiente: u8,
    pub nombre: String
}

#[derive(Eq, Debug, Hash, PartialEq)]
pub enum Dia {
    Domingo,
    Lunes,
    Martes,
    Miercoles,
    Jueves,
    Viernes,
    Sabado
}

impl Dia {
    pub fn value(&self) -> Day {
        match *self {
            Dia::Domingo => Day {nombre: "Dom".to_owned(), actual: 1, siguiente: 2},
            Dia::Lunes => Day {nombre: "Lun".to_owned(), actual: 2, siguiente: 3},
            Dia::Martes => Day {nombre: "Mar".to_owned(), actual: 3, siguiente: 4},
            Dia::Miercoles => Day {nombre: "Mie".to_owned(), actual: 4, siguiente: 5},
            Dia::Jueves => Day {nombre: "Jue".to_owned(), actual: 5, siguiente: 6},
            Dia::Viernes => Day {nombre: "Vie".to_owned(), actual: 6, siguiente: 7},
            Dia::Sabado => Day {nombre: "Sab".to_owned(), actual: 7,  siguiente: 1},
        }
    }
}

impl Display for Dia {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", self.value().nombre) 
    }
}