#![feature(linked_list_cursors)]
use std::collections::LinkedList;

use rand::prelude::SliceRandom;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumIter)]
enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
enum Value {
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Card {
    suit: Suit,
    value: Value,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.suit == other.suit {
            Some(self.value.cmp(&other.value))
        } else {
            None
        }
    }
}

fn get_deck() -> Vec<Card> {
    Suit::iter()
        .map(|suit| Value::iter().map(move |value| Card { suit, value }))
        .flatten()
        .collect()
}

fn patience(deck: impl IntoIterator<Item = Card>) -> usize {
    let mut deck: LinkedList<Card> = deck.into_iter().collect();
    let mut cursor_mut = deck.cursor_front_mut();
    while cursor_mut.current().is_some() {
        let cursor = cursor_mut.as_cursor();
        let can_be_removed = match [cursor.peek_prev(), cursor.peek_next()] {
            [Some(card1), Some(card2)] => card1.suit == card2.suit || card1.value == card2.value,
            _ => false,
        };
        if can_be_removed {
            cursor_mut.move_prev();
            cursor_mut.remove_current().unwrap();
        } else {
            cursor_mut.move_next()
        }
    }
    deck.len()
}

fn main() {
    const NUMBER_OF_GAMES: usize = 50_000_000;
    let deck = get_deck();

    #[derive(Clone, Copy)]
    struct Stats {
        total_len: usize,
        total_successes: usize,
    }

    let Stats {
        total_len,
        total_successes,
    } = (0..NUMBER_OF_GAMES)
        .into_par_iter()
        .map(|_| {
            let mut shuffled_deck = deck.clone();
            shuffled_deck.shuffle(&mut rand::thread_rng());
            patience(shuffled_deck)
        })
        .map(|len| Stats {
            total_len: len,
            total_successes: (len == 2) as usize,
        })
        .reduce_with(|s1, s2| Stats {
            total_len: s1.total_len + s2.total_len,
            total_successes: s1.total_successes + s2.total_successes,
        })
        .unwrap();
    println!(
        "Success rate: {total_successes}/{NUMBER_OF_GAMES} ({percentage}% or 1 in {one_in:.2})",
        percentage = (total_successes * 100) as f64 / NUMBER_OF_GAMES as f64,
        one_in = NUMBER_OF_GAMES as f64 / total_successes as f64,
    );
    println!("Average len: {}", total_len as f64 / NUMBER_OF_GAMES as f64);
}
