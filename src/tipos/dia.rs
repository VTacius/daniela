#[derive(Debug)]
pub struct Day {
    pub actual: u8,
    pub siguiente: u8
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
            Dia::Domingo => Day {actual: 1, siguiente: 2},
            Dia::Lunes => Day {actual: 2, siguiente: 3},
            Dia::Martes => Day {actual: 3, siguiente: 4},
            Dia::Miercoles => Day {actual: 4, siguiente: 5},
            Dia::Jueves => Day {actual: 5, siguiente: 6},
            Dia::Viernes => Day {actual: 6, siguiente: 7},
            Dia::Sabado => Day {actual: 7, siguiente: 1},
        }
    }

    pub fn set(frase: &str) -> Dia {
        match frase {
            "Dom" => Self::Domingo,
            "Lun" => Self::Lunes,
            "Mar" => Self::Martes,
            "Mie" => Self::Miercoles,
            "Jue" => Self::Jueves,
            "Vie" => Self::Viernes,
            "Sab" => Self::Sabado,
            &_ => todo!()
        }
    }
}