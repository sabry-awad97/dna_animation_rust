use rand::Rng;
use std::{thread, time};

const PAUSE: u64 = 150;
const ROWS: [&str; 18] = [
    "         ##",
    "        #{}-{}#",
    "       #{}---{}#",
    "      #{}-----{}#",
    "     #{}------{}#",
    "    #{}------{}#",
    "    #{}-----{}#",
    "     #{}---{}#",
    "     #{}-{}#",
    "      ##",
    "     #{}-{}#",
    "     #{}---{}#",
    "    #{}-----{}#",
    "    #{}------{}#",
    "     #{}------{}#",
    "      #{}-----{}#",
    "       #{}---{}#",
    "        #{}-{}#",
];

struct DnaAnimation {}

impl DnaAnimation {
    fn animate(&self) {
        let mut row_index = 0;
        loop {
            row_index = (row_index + 1) % ROWS.len();

            if row_index == 0 || row_index == 9 {
                println!("{}", ROWS[row_index]);
                continue;
            }

            let (left_nucleotide, right_nucleotide) = match rand::thread_rng().gen_range(1..5) {
                1 => ("A", "T"),
                2 => ("T", "A"),
                3 => ("C", "G"),
                _ => ("G", "C"),
            };

            println!(
                "{}",
                ROWS[row_index]
                    .replace("{}", &left_nucleotide)
                    .replace("{}", &right_nucleotide)
            );
            thread::sleep(time::Duration::from_millis(PAUSE));
        }
    }
}

fn main() {
    println!("DNA Animation");
    println!("Press Ctrl-C to quit...");

    DnaAnimation {}.animate();
}
