# ⏰ Decifrando o Jutsu Temporal: Clock em Rust! ⏰

E aí, Shinobis do Código! 🍜🦊 

Tá preparado pra dominar o tempo como o próprio Sasuke com o Rinnegan? DATTEBAYO! Hoje vamos entender esse jutsu INSANO que manipula horas e minutos como se fossem meros clones das sombras! 

## 🕙 O Pergaminho do Tempo (O Código Explicado)

Vamos analisar esse jutsu temporal linha por linha, como se estivéssemos estudando um pergaminho proibido do clã Nara:

```rust
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    total_minutes: i32,
}
```

### 🍥 Invocando os Jutsus Básicos (Bibliotecas)

* `std::fmt`: Este é como o Jutsu de Transformação básico! Permite que nosso relógio se transforme em uma string bonitinha.
* `std::ops::{Add, Sub}`: São tipo os selos de mão para adicionar e subtrair - essenciais para manipular o tempo!

### ⚡ A Estrutura Clock: O Coração do Jutsu

Nossa struct `Clock` é simples como o Naruto (mas muito mais inteligente, acredite!). Ela guarda apenas `total_minutes` como um `i32`. É tipo o chakra do nosso jutsu temporal - tudo o que precisamos!

As derivações `Debug`, `PartialEq` e `Eq` são tipo kekkei genkai automáticos:
- `Debug`: Permite imprimir o relógio para debug (tipo o Byakugan, mas para código!)
- `PartialEq` e `Eq`: Permite comparar dois relógios (como comparar o chakra de dois ninjas!)

```rust
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_per_day = 24 * 60;
        let raw = hours * 60 + minutes;
        let normalized = ((raw % minutes_per_day) + minutes_per_day) % minutes_per_day;
        Self { total_minutes: normalized }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.total_minutes + minutes)
    }
}
```

### 🌪️ O Construtor: Formando o Selo Temporal!

O método `new` é o verdadeiro gênio aqui! Ele faz algo que até o Shikamaru ficaria impressionado:

1. Calcula `minutes_per_day = 24 * 60 = 1440` (o número de minutos em um dia)
2. Converte horas para minutos e soma com os minutos fornecidos: `raw = hours * 60 + minutes`
3. Normaliza os minutos para garantir que fiquem dentro do intervalo [0, 1439]:
   ```rust
   ((raw % minutes_per_day) + minutes_per_day) % minutes_per_day
   ```
   
   Isso é tipo o Kamui do Kakashi - mesmo se você passar números negativos ou gigantes, ele SEMPRE vai te transportar para o lugar certo no ciclo de 24 horas! 🤯

### ⏱️ Adicionando Minutos: O Jutsu da Expansão Temporal!

O método `add_minutes` é simples mas poderoso:

```rust
pub fn add_minutes(&self, minutes: i32) -> Self {
    Self::new(0, self.total_minutes + minutes)
}
```

Ele pega os minutos atuais, adiciona os novos minutos (que podem ser positivos ou negativos, como um genjutsu reversível!) e usa o construtor `new` para normalizar tudo. GENIAL! É tipo usar um clone para fazer o trabalho pesado enquanto você fica suave!

### 🔄 Sobrecarga de Operadores: Jutsus Avançados!

```rust
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
```

Essas implementações são tipo jutsus proibidos - permitem usar os operadores `+` e `-` diretamente com nosso relógio!

- `Clock + 30` vai adicionar 30 minutos 
- `Clock - 15` vai subtrair 15 minutos

É como ter um Rasengan e um Chidori no mesmo jutsu! 😱

### 📝 Display: O Jutsu de Apresentação!

```rust
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.total_minutes / 60;
        let minutes = self.total_minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
```

Este método é como o Jutsu de Transformação final! Transforma nosso relógio interno de minutos em uma string bonitinha no formato "HH:MM"!

1. Divide os minutos totais por 60 para obter as horas
2. Usa o resto da divisão para obter os minutos
3. Formata ambos com dois dígitos, adicionando zeros à esquerda se necessário (tipo `01:05`)

## 🍥 Exemplos de Como Usar Este Jutsu Temporal

```rust
// Criar um relógio normal
let clock = Clock::new(9, 30); // 09:30

// Lidando com overflow de horas
let late_night = Clock::new(23, 59); // 23:59
let one_minute_later = late_night + 1; // 00:00

// Lidando com números negativos
let weird_time = Clock::new(-5, -30); // 18:30

// Adicionando e subtraindo
let lunch = Clock::new(12, 0); // 12:00
let dinner = lunch + 420; // 19:00 (7 horas depois)
let breakfast = dinner - 840; // 05:00 (14 horas antes)
```

## 🔥 Por que este código é INCRÍVEL?

Este código é mais limpo que os jutsus de água do Tobirama! Ele resolve o problema com apenas UMA variável interna (`total_minutes`) e usa matemática inteligente para normalizar o tempo.

Além disso, ele implementa operadores para tornar o código mais legível - é tipo ativar o Sharingan e ver tudo em câmera lenta! 👁️‍🗨️

Agora você pode manipular o tempo como um verdadeiro Hokage dos códigos! Acredite!!! 🦊🍜 