#[derive(Debug, Copy, Clone)]
pub struct Tick {
    pub inicio: u16,
    pub fin: u16
}

impl Tick {
    pub fn new(inicio: u16, fin: u16) -> Tick {
        let inicio = inicio;
        let fin = fin;
        Tick { inicio, fin }
    }

    pub fn contiene(&self, ts: u16) -> bool {
        self.inicio <= ts && ts <= self.fin
    }

    pub fn solapa(&self, elemento: Tick) -> bool{
        (self.contiene(elemento.inicio) || self.contiene(elemento.fin)) || (elemento.contiene(self.inicio) && elemento.contiene(self.fin))
    }

    pub fn merge(&self, elemento: Tick) -> Tick {
        match (self.inicio <= elemento.inicio, self.fin >= elemento.fin) {
            (false, false) => Tick::new(elemento.inicio, elemento.fin),
            (false, true) => Tick::new(elemento.inicio, self.fin),
            (true, false) => Tick::new(self.inicio, elemento.fin),
            (true, true) => Tick::new(self.inicio, self.fin),
        }

    }
}


impl PartialEq for Tick {
    fn eq(&self, other: &Self) -> bool {
       self.inicio == other.inicio && self.fin == other.fin 
    }
}

impl Eq for Tick {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pertenece() {
        let rango = Tick::new(480, 540);
        assert!(rango.contiene(510));
    }

    #[test]
    fn test_no_pertenece() {
        let rango = Tick::new(480, 540);
        assert!(!rango.contiene(450));
    }

    #[test]
    fn test_solapa(){
        let base = Tick::new(480, 540);
        let comparativa = Tick::new(510, 570);
        assert!(base.solapa(comparativa));
    }
    
    #[test]
    fn test_comparado_solapa_internamente_en_base(){
        let base = Tick::new(480, 570);
        let comparativa = Tick::new(510, 540);
        assert!(base.solapa(comparativa));
    }
    
    #[test]
    fn test_base_solapa_internamente_en_comparado(){
        let comparativa = Tick::new(480, 570);
        let base = Tick::new(510, 540);
        assert!(base.solapa(comparativa));
    }
    
    #[test]
    fn test_no_solapa(){
        let base = Tick::new(480, 540);
        let comparativa = Tick::new(570, 600);
        assert!(!base.solapa(comparativa));
    }

    #[test]
    fn test_merge_inicio(){
        let base = Tick::new(480, 540);
        let agregado = Tick:: new(480, 570);
        assert_eq!( base.merge(agregado), Tick::new(480, 570));
    }

    #[test]
    fn test_merge_final(){
        let base = Tick::new(480, 570);
        let agregado = Tick:: new(450, 570);
        assert_eq!( base.merge(agregado), Tick::new(450, 570));
    }
    
    #[test]
    fn test_merge_base_contiene_agregado(){
        let base = Tick::new(420, 570);
        let agregado = Tick:: new(450, 540);
        assert_eq!( base.merge(agregado), base);
    }
    
    #[test]
    fn test_merge_agregado_contiene_base(){
        let base = Tick:: new(450, 540);
        let agregado = Tick::new(420, 570);
        assert_eq!( base.merge(agregado), agregado);
    }
}