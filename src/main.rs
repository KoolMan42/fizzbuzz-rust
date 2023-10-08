use std::collections::HashMap;

fn main() {
    // Numbers from 1 to 25
    let mut fizzy_map = HashMap::new();
    fizzy_map.insert(3, "Fizz".to_string());
    fizzy_map.insert(5, "Buzz".to_string());
    // fizzy_map.insert(2, "Divizy".to_string());
    for num in 1..=25 {
        let mut ans = String::new();
        for (k, v) in &fizzy_map {
            if num % k == 0 {
                ans.push_str(v);
            }
        }
        if ans.is_empty() {
            ans.push_str(&*num.to_string());
        }
        println!("{}", ans);
    }
}
