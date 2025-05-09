use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    total_minutes: i32,
}


impl Clock {
    /// Cria um Clock normalizando horas e minutos para o intervalo [00:00, 23:59].
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_per_day = 24 * 60;
        let raw = hours * 60 + minutes;
        // normaliza mesmo se raw for negativo ou >= 1440
        let normalized = ((raw % minutes_per_day) + minutes_per_day) % minutes_per_day;
        Self { total_minutes: normalized }
    }

    /// Retorna um novo Clock com `minutes` (podem ser negativos) adicionados.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.total_minutes + minutes)
    }
}

impl Add<i32> for Clock {
    type Output = Clock;
    fn add(self, minutes: i32) -> Clock {
        self.add_minutes(minutes)
    }
}

impl Sub<i32> for Clock {
    type Output = Clock;
    fn sub(self, minutes: i32) -> Clock {
        self.add_minutes(-minutes)
    }
}

// Impl de Display para formatar como "HH:MM"
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.total_minutes / 60;
        let minutes = self.total_minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
