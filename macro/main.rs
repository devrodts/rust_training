// The println macro
// println "prints" (displays) information to the console and are used for debugging purposes.
// It is a macro that takes a format string and a variable number of arguments, and prints the formatted string to the standard output.

let life = 99;
// The println! macro is used to print formatted strings to the console.
// The first argument is a format string, which can contain placeholders for variables.
println!("You have {} lives left", life);

// The second argument is the value of the variable life, which will replace the placeholder in the format string.
// The println! macro automatically adds a newline character at the end of the output.
println!("{:?} {:?}", life, life);

// The {:?} format specifier is used to print the value of the variable in a debug format, which is useful for debugging purposes.
// The debug format is a more detailed representation of the value, which can be useful for complex data types.
println!("{life:?}");

// The {} format specifier is used to print the value of the variable in a default format, which is suitable for most types.
// The default format is a more human-readable representation of the value, which is suitable for most types.
println!("{life}");