pub fn raindrops(n: u32) -> String {
    let  output = [(3, "Pling"), (5, "Plang"), (7, "Plong")].iter()
        .filter(|x| n % x.0 == 0) 
        .map(|x| x.1)
        .collect::<String>();

    if output.is_empty() {
        return n.to_string();
    }    
    output
}
