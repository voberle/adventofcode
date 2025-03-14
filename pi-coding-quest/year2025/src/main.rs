use rust_decimal::Decimal;

// Day          Price ($)          Ticker
const INPUT: &str = r"1            150.00             TLM
2            93.23              PIH
3            300.50             MTH
4            420.75             IUV
5            3.14               GST
6            720.20             FKE
7            12.57              KVW
8            88.90              TEC
9            210.00             OIL
10           2.64               PHI
11           45.60              CUV
12           33.83              SPI
13           999.99             MEME
14           28.27              MED
15           123.45             BIA
16           65.80              REN
17           6.53               HST
18           250.00             AND
19           18.85              YVO
20           33.33              XOR
21           8.46               NUM
22           777.77             POT
23           9.42               BNO
24           199.99             NOT
25           15.92              SPI
26           850.00             VSL
27           19.94              IVA
28           58.97              GST
29           27.95              PHI
30           21.99              EXW";

const PI: &str = "31415926535897932384626433832795";

#[derive(Debug)]
struct StockEntry {
    day: u64,
    price: Decimal,
    ticker: String,
}

impl StockEntry {
    fn price_as_string(&self) -> String {
        self.price.to_string().replace('.', "")
    }
}

fn build(input: &str) -> Vec<StockEntry> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            let day = parts[0].parse().unwrap();
            let price = Decimal::from_str_exact(parts[1]).unwrap();
            let ticker = parts[2].to_string();
            StockEntry { day, price, ticker }
        })
        .collect()
}

fn find_secret_code(dataset: &[StockEntry], number: &str) -> String {
    let code = dataset.iter().fold(Decimal::ZERO, |acc, e| {
        let price_str = e.price_as_string();
        if number.contains(&price_str) {
            if acc.is_zero() {
                // First day
                e.price
            } else if e.day % 2 == 0 {
                // Even day
                acc * e.price
            } else if e.day % 2 == 1 {
                // Odd day
                acc / e.price
            } else {
                panic!("bug")
            }
        } else {
            acc
        }
    });
    // No dot, no zero at end or beginning, and only first 10 digits.
    let clean_code = code
        .to_string()
        .replace('.', "")
        .trim_matches('0')
        .to_string();
    if clean_code.len() >= 10 {
        clean_code[0..10].to_string()
    } else {
        clean_code
    }
}

fn main() {
    let dataset = build(INPUT);

    assert_eq!(PI.len(), 32);
    println!("Code: {}", find_secret_code(&dataset, PI));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Day          Price ($)          Ticker
    const INPUT_TEST: &str = r"1            0.3                GST
2            3.75               JVW
3            2.64               SPI
4            7.77               LCK
5            1.6                HST
6            1.8                NUM
7            5.88               XIJ";

    const PHI: &str = "1618033988";

    #[test]
    fn test_part1() {
        let dataset = build(INPUT_TEST);

        assert_eq!(PHI.len(), 10);
        assert_eq!(find_secret_code(&dataset, PHI), "3375");
    }
}
