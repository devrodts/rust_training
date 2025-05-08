// Converta a data/hora inicial para PrimitiveDateTime.
// Crie uma Duration de 1.000.000.000 segundos.
// Adicione essa Duration à data/hora inicial.
// Formatação do Resultado:
// Use a função format para exibir a nova data/hora no formato YYYY-MM-DD HH:MM:SS.

use time::{Duration, PrimitiveDateTime};
use time::macros::{datetime, format_description};

// 1 gigasegundo = 10^9 s
const GIGASECOND: i64 = 1_000_000_000;

pub fn gigasecond(start: PrimitiveDateTime) -> PrimitiveDateTime {
    start + Duration::seconds(GIGASECOND)
}

fn main() {

    let start  = datetime!(2015-01-24 22:00:00);
    let result = gigasecond(start);

    let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
    println!("{}", result.format(&fmt).unwrap());   // 2046-10-02 23:46:40
    
}
