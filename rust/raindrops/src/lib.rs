pub fn raindrops(n: u32) -> String {
    let arr = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut s = String::new();

    for e in arr.iter() {
        if n % e.0 == 0 {
            s.push_str(e.1);
        }
    }

    if s.is_empty() {
        n.to_string()
    } else {
        s
    }
}
