pub fn raindrops(n: i32) -> String {
    let mut s: String = "".to_owned();
    let string_3   : &str = "Pling";
    let string_5   : &str = "Plang";
    let string_7   : &str = "Plong";
    let string_15  : &str = "PlingPlang";
    let string_21  : &str = "PlingPlong";
    let string_35  : &str = "PlangPlong";
    let string_105 : &str = "PlingPlangPlong";
    
    if n % 105 == 0 {
        s.push_str(string_105);  
    } else if n % 35 == 0 {
        s.push_str(string_35);
    } else if n % 21 == 0 {
        s.push_str(string_21);  
    } else if n % 15 == 0 {
        s.push_str(string_15)
    } else if n % 7 == 0 {
        s.push_str(string_7);  
    } else if n % 5 == 0 {
        s.push_str(string_5);  
    } else if n % 3 == 0 {
        s.push_str(string_3);
    } 
    else {
        s = n.to_string();
    }
    format!("{}", s)
}