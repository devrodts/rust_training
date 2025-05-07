fn verify_package_length(length: i32) -> <i32 as std::ops::Add>::Output {
    let mut package_length = 0;
    if length > 0 {
        package_length = length + 1;
    }
    package_length
}

fn verify_package_length(length: i32) -> <i32 as std::ops::Add>::Output {
    let mut package_length = 0;
    if length > 0 {
        package_length = length + 1;
    }
    package_length
}


fn main() {
    let length = 5;
    let package_length = verify_package_length(length);
    println!("Package length: {}", package_length);
}

// This function is used to verify the package length
// It checks if the length is greater than 1500 or less than 0
fn verify_package_lengthold(length: i32){
    if length > 1500{
        println!("Package length is too long: {}", length);
    } else if length < 0 {
        println!("Package length is negative: {}", length);
    }else {
        println!("Package length is valid: {}", length);
    }
}