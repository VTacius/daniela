mod tipos;
mod parser;

use tipos::Cronograma;
use tipos::Horario;

use pgx::*;

pg_module_magic!();

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
fn mostrar_cronograma(cronograma: Cronograma) -> String {
    cronograma.sentencias.join("\n")
}

#[pg_extern]
fn crear_cronograma(contenido: &str)  -> Cronograma {
    let horarios: Vec<Horario> = match parser::parsear_contenido(contenido) {
        Ok(horarios) => horarios,
        Err(_e) => {panic!("Error que aun no mostraremos")}
    };
    
    Cronograma::new(horarios)
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
