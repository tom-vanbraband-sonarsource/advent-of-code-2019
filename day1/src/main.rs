use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result : i64 = read().iter()
        .map(|&x| calculate(x) as i64)
        .sum();
    println!("{}", result);
}

fn calculate(mass: f64) -> f64 {
    let fuel = (mass / 3.0).floor() - 2.0;

    if fuel < 0.0 {
        return 0.0;
    }

    fuel + calculate(fuel)
}

fn read() -> Vec<f64> {
    let filename = "src/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|res| res.map(|line| line.parse::<f64>().unwrap()))
        .map(|x| x.unwrap())
        .collect::<Vec<f64>>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(calculate(12.0), 2.0);
        assert_eq!(calculate(14.0), 2.0);
        assert_eq!(calculate(1969.0), 966.0);
        assert_eq!(calculate(100756.0), 50346.0);
    }
}
