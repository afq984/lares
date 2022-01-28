fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(args.len(), 2, "expected exactly 1 argument");
    let history_json = read_words("history.json");
    let words_json = read_words("words.json");

    let answer = &args[1];

    let words: Vec<&str> = history_json
        .iter()
        .map(String::as_str)
        .chain(words_json.iter().map(String::as_str))
        .collect();

    if answer == "benchmark" {
        benchmark(
            &words,
            &history_json.iter().map(String::as_str).collect(),
            &vec!["lares"],
        );
    } else {
        assert_eq!(answer.bytes().count(), 5);
        run(&words, answer, &vec!["lares"]);
    }
}

fn benchmark(words: &Vec<&str>, history: &Vec<&str>, initial: &Vec<&str>) {
    let mut counts = 0;
    for (i, answer) in history.iter().enumerate() {
        counts += run(&words, answer, initial);
        println!(
            "benchmark {}/{}, avg {}",
            i + 1,
            history.len(),
            counts as f64 / (i as f64 + 1.)
        );
    }
    println!("average {}", counts as f64 / history.len() as f64);
}

fn run(words: &Vec<&str>, answer: &str, initial: &Vec<&str>) -> u32 {
    let mut attempts: u32 = 0;

    let mut candidates: Vec<&str> = words.clone();

    loop {
        let g: &str = match initial.get(attempts as usize) {
            Some(&x) => x,
            None => guess(&words, &candidates),
        };

        attempts += 1;
        println!("guess {:?}", g);

        if g == answer {
            break;
        }

        let c = compare(answer, g);
        eliminate(&mut candidates, g, c);
    }

    println!("{} attempts for {}", attempts, answer);

    attempts
}

fn read_words(path: &str) -> Vec<String> {
    let file = std::fs::File::open(path).unwrap();
    let buf_reader = std::io::BufReader::new(file);
    serde_json::from_reader(buf_reader).unwrap()
}

fn compare(answer: &str, guess: &str) -> u8 {
    let iter = answer.bytes().zip(guess.bytes());
    let mut result: u8 = 0;
    let mut extras = Vec::with_capacity(guess.bytes().count());
    let mut mult = 1;
    for (a, g) in iter {
        if a == g {
            result += 2 * mult;
        } else {
            extras.push(a);
        }
        mult *= 3;
    }

    let iter = answer.bytes().zip(guess.bytes());
    let mut mult = 1;
    for (a, g) in iter {
        if a != g {
            match extras.iter().position(|&x| x == g) {
                None => (),
                Some(i) => {
                    result += 1 * mult;
                    extras[i] = 0;
                }
            }
        }
        mult *= 3;
    }

    result
}

#[cfg(test)]
mod tests {
    fn pack(a: u8, b: u8, c: u8, d: u8, e: u8) -> u8 {
        a + (b * 3) + (c * 9) + (d * 27) + (e * 81)
    }

    #[test]
    fn compare() {
        assert_eq!(super::compare("hello", "level"), pack(1, 2, 0, 0, 1));
        assert_eq!(super::compare("world", "hello"), pack(0, 0, 0, 2, 1));
    }
}

fn guess<'a>(words: &'a Vec<&str>, candidates: &Vec<&str>) -> &'a str {
    let mut best_guess: &'a str = &words[0];
    let mut best_exp = f64::MAX;

    for guess in words.iter() {
        let mut groups: Vec<u64> = vec![0; 243];
        for assumed_answer in candidates.iter() {
            if guess != assumed_answer {
                groups[compare(assumed_answer, guess) as usize] += 1;
            }
        }

        let mut exp = 0.;
        for group in groups {
            exp += (group as f64).powf(2.5);
        }

        if exp < best_exp {
            best_exp = exp;
            best_guess = guess;
        }
    }

    best_guess
}

fn eliminate(candidates: &mut Vec<&str>, guess: &str, c: u8) {
    candidates.retain(|&x| compare(&x, guess) == c);
}
