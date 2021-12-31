pub fn raindrops(n: usize) -> String {
    let arr = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let mut s = String::new();

    for e in arr.iter() {
        if n % e.0 == 0 {
            s.push_str(e.1);
        }
    }

    if s.capacity() == 0 {
        s.push_str(&n.to_string());
    }

    return s;
}
