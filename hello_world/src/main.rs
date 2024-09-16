fn concat_strings(s1: &String, s2: &String) -> String {
   let mut word = s1.to_string(); //Creating a new variable word, that is mutable...and assigned the value s1
   word.push_str(s2); //Pushed back s2 string into s1 to concatenate word into one sentence.

   return word; //Returning string variable word.
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}