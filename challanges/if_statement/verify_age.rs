fn verify_age(age: i32) -> String{
    if age < 18 {
        String::from("You are a minor.")
        else if age >= 18 && age < 60 {
            String::from("You are an adult.")
        } else if age >= 100 {
            String::from("You are a senior citizen.")
        } else {
            String::from("Invalid age, are you alive?")
        }
    }
}