Yo, Shinobi da Programação! 🍥

Escuta aqui, 'ttebayo! Chegou uma missão nível A pra você: dominar a arte ninja de **Reverter Strings** em Rust! Pode parecer moleza como o Exame Chunin, mas até os Kages precisam conhecer as técnicas certas pra não acabar preso num Genjutsu de código!

Se liga nesse pergaminho de código que encontramos:

**Rust**

```
// Assinatura do Jutsu Secreto (com um errinho de digitação, hehe)
pb fn reverse_string(input: &str) -> String{
    // Isso aqui é tipo um Kage Bunshin que ainda não foi invocado!
    todo!("Preciso invocar meu Jutsu pra reverter essa string: {input}");

    // --- Pergaminhos de Técnicas ---

    // Técnica Secreta da Folha: O Rasengan de Iteradores! (Quase lá!)
    // using one line
    // input.chars.rev().collect::<String>(); // Precisa de '()' e ser o retorno!

    // Técnica do Taijutsu Direto: O Loop da Determinação! (Funciona, dattebayo!)
    // using a for loop
    // let mut reversed = String::new();
    // for c in input.chars().rev(){
    //     reversed.push(c);
    // }
    // reversed

    // Técnica do Genjutsu Complicado: O Loop Infinito? (Cuidado, Shinobi!)
    // using a while loop
    // let mut reversed_while = String::new();
    // let mut i = input.len() as isize - 1;
    // while i >= 0 {
    //     // Cuidado! Usar nth() no loop é como lutar contra o Madara sem o Sharingan! Lento!
    //     reversed_while.push(input.chars().nth(i as usize).unwrap());
    //     i -= 1;
    // }
    // reversed_while
}
```

Relaxa, nem todo Jutsu tá perfeito de primeira! Aquele `todo!` ali significa que a missão ainda tá incompleta. Mas não se preocupa, o Sábio dos Seis Caminhos (do Rust) nos deixou várias formas de completar essa tarefa!

---

### Jutsu Nível Jounin: O Rasengan dos Iteradores! ✨

Essa é a técnica mais **chakra-eficiente** e elegante em Rust, 'ttebayo! É direta, poderosa e mostra que você manja dos paranauês da Vila da Folha (do Rust, claro!).

**Como funciona o Jutsu:**

1. `input.chars()`: Quebra a string em seus caracteres individuais, como Shurikens prontas para o ataque!
2. `.rev()`: **Aqui tá o segredo!** Igualzinho ao Kage Bunshin reverso, ele inverte a ordem dos caracteres. BOOM! 💥
3. `.collect::<String>()`: Junta todos os caracteres de volta numa nova String, mais poderosa e invertida! É a finalização!

**Pergaminho Completo (Código Corrigido):**

**Rust**

```
/// Reverte uma string usando a técnica ninja dos iteradores.
/// É rápido, eficiente e mostra seu Nindō em Rust! Dattebayo!
pub fn reverse_string_iterator(input: &str) -> String {
    // Concentra o Chakra... e RASENGAN!
    input.chars().rev().collect::<String>()
}

// Exemplo de como usar:
// let original = "Naruto Uzumaki";
// let invertida = reverse_string_iterator(original);
// println!("{} ao contrário é {}", original, invertida); // ikamuzU oturaN
```

---

### Jutsu Nível Chunin: O Loop da Determinação (Taijutsu!) 💪

Essa técnica é mais direta, passo a passo, como um bom treino de Taijutsu com o Mestre Guy! Você constrói a string invertida caractere por caractere.

**Como funciona o Jutsu:**

1. `let mut reversed = String::new();`: Prepara um pergaminho (String) vazio pra escrever o resultado.
2. `for c in input.chars().rev()`: Pega cada caractere da string original, mas já na ordem invertida (`.rev()`).
3. `reversed.push(c);`: Adiciona o caractere invertido no final do nosso pergaminho.
4. `reversed`: Quando o loop acaba, o pergaminho tá completo com a string invertida! Missão cumprida!

**Pergaminho Completo (Código Corrigido):**

**Rust**

```
/// Reverte uma string usando a força de vontade e um loop for.
/// É como treinar Taijutsu: direto e eficaz!
pub fn reverse_string_for_loop(input: &str) -> String {
    let mut reversed = String::new(); // Começa vazio
    // Pega os chars de trás pra frente
    for c in input.chars().rev() {
        reversed.push(c); // Adiciona no novo pergaminho
    }
    reversed // Retorna o resultado!
}

// Exemplo de como usar:
// let original = "Sasuke Uchiha";
// let invertida = reverse_string_for_loop(original);
// println!("{} ao contrário é {}", original, invertida); // ahihcU ekusaS
```

---

### Jutsu Nível Genin: O Loop Complicado (Genjutsu?) 😵‍💫

Essa técnica com `while` funciona, mas exige mais controle manual e tem uma armadilha escondida! Usar `nth(i)` dentro do loop pra pegar um caractere é **MUITO** lento em Rust pra strings grandes, porque ele tem que percorrer a string toda vez! É como cair num Genjutsu que te deixa lento!

**Como funciona o Jutsu:**

1. `let mut reversed_while = String::new();`: Outro pergaminho vazio.
2. `let mut i = input.len() as isize - 1;`: Pega o índice do *último* caractere.
3. `while i >= 0`: Continua enquanto não chegarmos no começo da string.
4. `reversed_while.push(input.chars().nth(i as usize).unwrap());`: **CUIDADO!** Pega o caractere na posição `i` (lento!) e adiciona no pergaminho.
5. `i -= 1;`: Vai pro caractere anterior.
6. `reversed_while`: Retorna o resultado.

**Pergaminho Completo (Código Corrigido, mas não recomendado pra strings grandes):**

**Rust**

```
/// Reverte uma string com um loop while. Funciona, mas cuidado com a armadilha do `nth()`!
/// Pode ser lento como o Rock Lee sem os pesos!
pub fn reverse_string_while_loop(input: &str) -> String {
    let mut reversed_while = String::new();
    let chars: Vec<char> = input.chars().collect(); // Melhor coletar os chars uma vez só!
    let mut i = chars.len() as isize - 1;

    while i >= 0 {
        // Agora pega do Vec, que é bem mais rápido (acesso O(1))!
        reversed_while.push(chars[i as usize]);
        i -= 1;
    }
    reversed_while
}

// Exemplo de como usar:
// let original = "Sakura Haruno";
// let invertida = reverse_string_while_loop(original);
// println!("{} ao contrário é {}", original, invertida); // onuraH arukaS
```

*(Nota: Fizemos uma pequena melhoria no `while` loop coletando os caracteres em um `Vec` primeiro pra evitar a lentidão do `nth()` repetido!)*

---

É isso aí, Shinobi! Agora você tem três Jutsus pra reverter strings em Rust! O Rasengan dos Iteradores é geralmente o caminho do Hokage, mas conhecer as outras técnicas fortalece seu Nindō!

Continue treinando e nunca desista, 'ttebayo! 💪🍜
