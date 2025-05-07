//Crie um programa que solicite um número inteiro ao usuário.
// * Se o número for positivo, imprima "O número é positivo."
// * Se o número for negativo, imprima "O número é negativo."
// * Se o número for zero, imprima "O número é zero."
//   Utilize `if`, `else if` e `else`.

fn classify_number(num: i32) -> String {
    if num > 0{
        String::from("The number is positive.")
    }else if num < 0{
        String::from("The number is negative.")
    }else{
        String::from("The number is zero.")
    }
}