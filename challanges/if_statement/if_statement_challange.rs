fn if_statement_challenge(){

    // This function is used to verify the package length
    // It checks if the length is greater than 2048 or less than 0

    fn verify_package_length(length: i32){
        if length > 2048{
            println!("Package length is too long: {}", length);
        } else if length < 0 {
            println!("Package length is negative: {}", length);
        }else {
            println!("Package length is valid: {}", length);
        }
    }
    let length = 5;
    verify_package_length(length);
}