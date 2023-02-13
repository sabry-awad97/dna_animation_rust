use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct DnaAnimation {
    pause: Duration,
    rows: [&'static str; 18],
}

impl DnaAnimation {
    fn animate(&self, running: &AtomicBool) {
        let mut row_index = 0;
        while running.load(Ordering::SeqCst) {
            row_index = (row_index + 1) % self.rows.len();

            let row = if row_index == 0 || row_index == 9 {
                self.rows[row_index].to_owned()
            } else {
                let n = match rand::random::<u8>() % 4 {
                    0 => ('A', 'T'),
                    1 => ('T', 'A'),
                    2 => ('C', 'G'),
                    3 => ('G', 'C'),
                    _ => unreachable!(),
                };
                self.rows[row_index]
                    .replace("{}", &n.0.to_string())
                    .replace("{}", &n.1.to_string())
            };
            println!("{}", row);
            thread::sleep(self.pause);
        }
    }
}

fn main() {
    println!("DNA Animation");
    println!("Press Ctrl-C to quit...");

    let dna = DnaAnimation {
        pause: Duration::from_millis(150),
        rows: [
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
        ],
    };

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    println!("DNA Animation");
    println!("Press Ctrl-C to quit...");

    let handle = thread::spawn(move || {
        dna.animate(&running);
    });

    handle.join().expect("Error running DNA animation");
}
