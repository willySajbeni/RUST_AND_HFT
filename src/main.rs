use rand::Rng;


use std::thread;
use std::sync::mpsc;
use std::time::{Duration, Instant};

fn main() {
    let (tx, rx) = mpsc::channel();

    // Simulate spot dollar market (faster, more liquid)
    let tx1 = tx.clone();
    thread::spawn(move || {
        let mut spot = 5.00;
        loop {
            spot += offset();
            tx1.send(("spot", spot)).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    // Simulate futures dollar market (slightly delayed)
    let tx2 = tx.clone();
    thread::spawn(move || {
        let mut future = 5.03;
        loop {
            future += offset();
            tx2.send(("future", future)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    let mut spot = 0.0;
    let mut future = 0.0;
    let cost = 0.005; // operational cost per dollar
    let mut timer = Instant::now();

    for (market, price) in rx {
        match market {
            "spot" => spot = price,
            "future" => future = price,
            _ => {}
        }

        let spread = future - spot;
        if spread > cost {
            println!(
                "💰 Arbitrage detected: Buy spot at R${:.2} and sell future at R${:.2} | Gross profit: R${:.2}",
                spot, future, spread - cost
            );
        }

        if timer.elapsed().as_secs() >= 1 {
            println!("🟢 Spot: R${:.2} | 🔵 Future: R${:.2}", spot, future);
            timer = Instant::now();
        }
    }
}

// Simulated price fluctuation
fn offset() -> f64 {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_millis();
    let osc = (millis % 5) as f64 - 2.0;
    osc * 0.001 // fluctuation between -0.002 and +0.002
}
