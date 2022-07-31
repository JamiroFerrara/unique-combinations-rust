use itertools::Itertools;

fn main() {
    let characters = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d"];
    let n = 4;
    
    let combinations : Vec<_> = (2..n).fold(
        characters.iter().cartesian_product(characters.iter()).unique().map(|(&a, &b)| a.to_owned() + b).collect(),
        |acc, _| acc.into_iter().cartesian_product(characters.iter()).map(|(a, b)| a.to_owned() + b).collect()
    );

    let no_duplicates = remove_duplicates(&combinations);
    let unique_values = get_unique_values(&no_duplicates);

    println!("Tutte le combinazioni di grandezza {} caratteri da {} a {}", n, characters[0], characters[characters.len() - 1]);
    println!("{:?}", unique_values);
    println!("Combinazioni totali --> {}", unique_values.len());
}

fn is_equal_no_order(a: &String, b: &String) -> bool{
    if a.chars().sorted().eq(b.chars().sorted()){
        return true;
    } else {
        return false;
    }
}

fn has_duplicates(value: &String) -> bool{
    let chars = value.chars();
    let mut has_duplicates: bool = false;

    for c in chars.clone() {
        let mut once = false;
        for x in chars.clone() {
            if c == x {
                if once {
                    has_duplicates = true;
                    break
                } else {
                    once = true;
                }
            }
        }
    }
    return has_duplicates;
}

fn remove_duplicates(values: &Vec<String>) -> Vec<String>{
    let mut unique_values: Vec<String> = Vec::new();
    for value in values {
        if !has_duplicates(value){
            unique_values.push(value.to_owned());
        }
    }
    return unique_values;
}

fn get_unique_values(values: &Vec<String>) -> Vec<String>{
    let mut unique_values: Vec<String> = Vec::new();

    for value in values {
        for x in values {
            if !is_equal_no_order(value, x) {
                if !unique_values.contains(&String::from_iter(value.chars().sorted())) {
                    unique_values.push(value.to_owned());
                    break
                }
            }
        }
    }
    return unique_values;
}
