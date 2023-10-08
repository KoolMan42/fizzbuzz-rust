use std::collections::HashMap;

fn main() {
    // Numbers from 1 to 25
    let mut fizzy_map = HashMap::new();
    fizzy_map.insert(3, "Fizz".to_string());
    fizzy_map.insert(5, "Buzz".to_string());
    for num in 1..=25 {
        let mut ans = vec![];
        for(k,v) in &fizzy_map {
           if k % num == 0 {
               ans.push(v.clone());
           }
        }
        let ans2 = ans.join("");
        println!("{:?}", ans2)
    }
}
