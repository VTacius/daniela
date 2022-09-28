mod tipos;
mod parser;

use tipos::Cronograma;
use tipos::Horario;

use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello_daniela() -> &'static str {
    "Hello, daniela"
}

fn buscar_en_cronograma(cronograma: Cronograma, ts: pgx::Timestamp, permisivo: bool) -> bool {
    let dia = ts.weekday().number_from_sunday();
    let hash = (ts.hour() as u16 * 60) + ts.minute() as u16;
    match cronograma.horarios.get(&dia) {
       None => permisivo,
       Some(e) => e.iter().find(|t| t.contiene(hash)).is_some(),
    }
}

#[pg_extern]
fn en_cronograma(cronograma: Cronograma, ts: pgx::Timestamp) -> bool {
    buscar_en_cronograma(cronograma, ts, false)
}

#[pg_extern]
fn en_cronograma_permisivo(cronograma: Cronograma, ts: pgx::Timestamp) -> bool {
    buscar_en_cronograma(cronograma, ts, true)
}

#[pg_extern]
fn crear_cronograma(contenido: &str)  -> Cronograma {
    let sentencias: Vec<Horario> = contenido
        .split(&['\n', ','])
        .map(|linea|{
            parser::parsear_linea(linea.trim())
        })
        .collect();
    
    Cronograma::new(sentencias)
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_daniela() {
        assert_eq!("Hello, daniela", crate::hello_daniela());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
