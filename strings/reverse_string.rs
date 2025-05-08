pb fn reverse_string(input: &str) -> String{
    // using one line
    input.chars.rev().collect::<String>();

    // using a for loop
    let mut reversed = String::new();
    for c in input.chars().rev(){
        reversed.push(c);
    }
    reversed

    // using a while loop   
    let mut reversed_while = String::new();
    let mut i = input.len() as isize - 1;
    while i >= 0 {
        reversed_while.push(input.chars().nth(i as usize).unwrap());
        i -= 1;
    }
    reversed_while
}