use std::time::{UNIX_EPOCH, SystemTime};

struct Card {
    question: String,
    answer: String,
    update: u32,
    interval: u32,
    difficulty: f64,
}

#[allow(dead_code)]
fn today_in_secs() -> u32 {
    let seconds_in_day = 24 * 60 * 60;
    let difference = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime::duration_since failed");
    (difference.as_secs() / seconds_in_day) as u32
}

#[allow(dead_code)]
fn percentage_overdue(card: Card, today: u32) -> u32 {
    let calculated = (today - card.update) / card.interval;
    if calculated > 2 { 2 } else { calculated }
}

#[allow(dead_code)]
fn within_bounds(some_number: f64) -> f64 {
    if some_number < 0.0 {
        0.0
    } else if some_number > 1.0 {
        1.0
    } else {
        some_number
    }
}

fn main() {
    println!("{:?}", today_in_secs());
}
