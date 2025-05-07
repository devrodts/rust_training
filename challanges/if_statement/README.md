## 5 Exercícios de `if` Statement em Rust para Fixar seus Conhecimentos:

Para  solidificar seus conhecimentos em `if` statements na linguagem Rust, preparei 5 exercícios com diferentes níveis de complexidade.

---

**Exercício 1: Verificador de Maioridade**

Escreva um programa em Rust que peça ao usuário para inserir sua idade.

* Se a idade for maior ou igual a 18, imprima "Você é maior de idade."
* Caso contrário, imprima "Você é menor de idade."

---

**Exercício 2: Classificador de Números**

Crie um programa que solicite um número inteiro ao usuário.

* Se o número for positivo, imprima "O número é positivo."
* Se o número for negativo, imprima "O número é negativo."
* Se o número for zero, imprima "O número é zero."
  Utilize `if`, `else if` e `else`.

---

**Exercício 3: Pode ou Não Pode Dirigir?**

Desenvolva um programa que pergunte a idade do usuário e se ele possui carteira de motorista (responda com 's' para sim ou 'n' para não).

* Se o usuário tiver 18 anos ou mais **e** possuir carteira de motorista, imprima "Você pode dirigir."
* Se o usuário tiver 18 anos ou mais **mas não** possuir carteira, imprima "Você precisa tirar a carteira de motorista para dirigir."
* Se o usuário for menor de 18 anos, imprima "Você não pode dirigir."
  Considere o uso de `if`s aninhados ou operadores lógicos (`&&`).

---

**Exercício 4: `if` em uma Declaração `let`**

Escreva um programa que defina uma variável booleana `esta_chovendo`.

* Use uma declaração `let` com uma expressão `if` para atribuir uma string a uma nova variável chamada `clima`.
* Se `esta_chovendo` for `true`, `clima` deve receber "Leve um guarda-chuva!".
* Se `esta_chovendo` for `false`, `clima` deve receber "Aproveite o dia!".
* Imprima o valor da variável `clima`.

---

**Exercício 5: Calculadora Simples com `if`**

Crie um programa que peça ao usuário para inserir dois números e um operador aritmético (`+`, `-`, `*`, `/`).

* Utilize `if` e `else if` para verificar qual operador foi inserido.
* Realize a operação correspondente e imprima o resultado.
* Se o operador for inválido, imprima uma mensagem de erro.
* **Desafio extra:** Se o operador for `/` e o segundo número for 0, imprima "Erro: Divisão por zero!".

---

**Dicas para Resolver:**

* Lembre-se da sintaxe do `if` em Rust: `if condicao { ... } else if outra_condicao { ... } else { ... }`.
* Para ler a entrada do usuário, você pode usar a crate `std::io`.
* Converta a entrada do usuário para o tipo de dado esperado (por exemplo, de String para i32 para números inteiros).
* Teste seus programas com diferentes entradas para garantir que funcionam corretamente em todos os cenários.

Espero que estes exercícios te ajudem! Bons estudos em Rust!
