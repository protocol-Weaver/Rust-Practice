
fn concatenate_strings(string1 : &str, string2 : &str) -> String{
    let mut result = String::from(string1);
    result.push_str(string2);
    result
}

fn main() {
    
    let string1 : &str = "GoodNight";
    let string2 : &str = "World";
    let result : String = concatenate_strings(string1, string2);
    println!("{}",result);

}


