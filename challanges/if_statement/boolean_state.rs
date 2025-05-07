// Escreva um programa que defina uma variável booleana `esta_chovendo`.

// * Use uma declaração `let` com uma expressão `if` para atribuir uma string a uma nova variável chamada `clima`.
// * Se `esta_chovendo` for `true`, `clima` deve receber "Leve um guarda-chuva!".
// * Se `esta_chovendo` for `false`, `clima` deve receber "Aproveite o dia!".
// * Imprima o valor da variável `clima`.
// Crie uma variável booleana chamada `esta_chovendo` e atribua um valor a ela (true ou false).
fn boolean_state(){
   let raining = true;
   if raining{
         let weather = "Take an umbrella!";
         println!("{}", weather);
   }else{
         let weather = "Enjoy the day!";
         println!("{}", weather);
   }
}