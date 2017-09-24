use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Debug)]
struct Card {
    question: String,
    answer: String,
    update: u32,
    interval: u32,
    difficulty: f64,
}

fn today_in_secs() -> u32 {
    let seconds_in_day = 24 * 60 * 60;
    let difference = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("SystemTime::duration_since failed");
    (difference.as_secs() / seconds_in_day) as u32
}

fn percent_overdue(interval: u32, update: u32, today: u32) -> f64 {
    let calculated = (today - update) as f64 / interval as f64;
    if calculated > 2.0 { 2.0 } else { calculated }
}

fn within_bounds(some_number: f64) -> f64 {
    if some_number < 0.0 {
        0.0
    } else if some_number > 1.0 {
        1.0
    } else {
        some_number
    }
}

fn new_interval(rating: f64, difficulty_weight: f64, percent_overdue: f64) -> u32 {
    if difficulty_weight == 0.0 {
        1
    } else if rating == 1.0 {
        ((1.0 / difficulty_weight) / difficulty_weight).round() as u32
    } else {
        1 + (((difficulty_weight - 1.0) * percent_overdue).round() as u32)
    }
}

fn new_card(card: &Card, difficulty: f64, interval: u32, today: u32) -> Card {
    Card {
        question: card.question.clone(),
        answer: card.answer.clone(),
        difficulty: difficulty,
        interval: interval,
        update: today,
    }
}

fn calculate(card: &Card, rating: f64, today: u32) -> Card {
    let percent_overdue = percent_overdue(card.interval, card.update, today);

    let difficulty = within_bounds(card.difficulty + (8.0 - 9.0 * rating) * percent_overdue / 17.0);
    let difficulty_weight = 3.0 - 1.7 * difficulty;
    let new_interval = new_interval(rating, difficulty_weight, percent_overdue);
    new_card(&card, difficulty, new_interval, today)
}

fn main() {
    let card = Card {
        question: String::from("Question"),
        answer: String::from("Answer"),
        update: today_in_secs() - 17,
        interval: 100,
        difficulty: 0.2,
    };
    let updated_card = calculate(&card, 1.0, today_in_secs());
    println!("old card: {:?}", card);
    println!("new card: {:?}", updated_card);
}
