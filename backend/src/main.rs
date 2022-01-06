mod cards;
mod game;
mod stands;
mod usecases;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use warp::Filter;

use cards::models::Attribute;
use cards::models::Card;
use cards::models::CardAttribute;
use cards::models::Deck;

use std::convert::TryInto;
use std::str;
use std::str::Utf8Error;

use stands::stand::Stand;

use usecases::compare_two_cards_usecase;
use usecases::shuffle_deck_usecase;

use game::host_game;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    // let route_new_game = warp::path("newgame").map(|| "Creating new game");
    let route_new_game = warp::post()
        .and(warp::path("newgame"))
        .and(warp::path::end())
        .and(host_game::json_body())
        .and_then(host_game::create_room);

    warp::serve(route_new_game)
        .run(([127, 0, 0, 1], 9090))
        .await
}

fn test_game() {
    let mut deck = Deck::new_with(generate_cards());
    deck = shuffle_deck_usecase::execute(deck);
    let (cards1, cards2) = deck.split();
    play(cards1, cards2)
}

fn play(mut deck1: Deck, mut deck2: Deck) {
    while deck1.is_empty() == false && deck2.is_empty() == false {
        let mut draw_pile: Vec<&Card> = vec![];

        let a = deck1.draw();
        let b = deck2.draw();

        let winner = compare_two_cards_usecase::execute(&a, &b, &Attribute::random_attribute());
        if let Some(w) = winner {
            if w.get_id() == a.get_title() {
                // let ddd = *draw_pile.first().unwrap();
                // deck1.add_card(*ddd);
            } else {
                // deck2.add_card(draw_pile);
            }
            draw_pile.clear();
            println!(
                "Fight - POWER! {:?} vs {:?} -> {:?}",
                a.get_title(),
                b.get_title(),
                w.get_title()
            );
        } else {
            draw_pile.push(&a);
            draw_pile.push(&b);
        };
    }

    show_winner(deck1);
}

fn run_check<'a>(
    pile: &'a mut Vec<&'a Card>,
    deck1: &'a mut Deck,
    deck2: &'a mut Deck,
) -> &'a Vec<&'a Card> {
    // loop {
    //     let card1 = deck1.draw();
    //     let card2 = deck2.draw();
    //     let winner = compare_two_cards_usecase::execute(&card1, &card2, &Attribute::random_attribute());
    //     match winner {
    //         Some(w) => break,
    //         None => {
    //             pile.push(&card1)
    //         }
    //     }
    // }

    pile
}

fn show_winner(deck1: Deck) {
    let win = if deck1.is_empty() {
        "Player 1"
    } else {
        "Player 2"
    };
    println!("winner is {}!!!", win);
}

fn generate_cards() -> Vec<Card> {
    vec![
        Card::new(
            String::from("1"),
            String::from("Star Platinum"),
            String::from("Jotaro"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 5),
                CardAttribute::new(Attribute::SPEED, 5),
                CardAttribute::new(Attribute::RANGE, 5),
                CardAttribute::new(Attribute::STAMINA, 5),
                CardAttribute::new(Attribute::PRECISION, 5),
                CardAttribute::new(Attribute::DEVELOPMENT, 5),
            ],
        ),
        Card::new(
            String::from("2"),
            String::from("Hierophant Green"),
            String::from("Kakyoin"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 5),
                CardAttribute::new(Attribute::SPEED, 10),
                CardAttribute::new(Attribute::RANGE, 10),
                CardAttribute::new(Attribute::STAMINA, 4),
                CardAttribute::new(Attribute::PRECISION, 6),
                CardAttribute::new(Attribute::DEVELOPMENT, 10),
            ],
        ),
        Card::new(
            String::from("3"),
            String::from("Magician Red"),
            String::from("Avdol"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 5),
                CardAttribute::new(Attribute::SPEED, 6),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 8),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("4"),
            String::from("Eremit purple"),
            String::from("Joseph"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 5),
                CardAttribute::new(Attribute::SPEED, 6),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 8),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("5"),
            String::from("Silver chariot"),
            String::from("Polnareff"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 5),
                CardAttribute::new(Attribute::SPEED, 6),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 8),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("6"),
            String::from("The World"),
            String::from("DIO"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 10),
                CardAttribute::new(Attribute::SPEED, 10),
                CardAttribute::new(Attribute::RANGE, 10),
                CardAttribute::new(Attribute::STAMINA, 10),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 9),
            ],
        ),
        Card::new(
            String::from("7"),
            String::from("Death 13"),
            String::from("Baby"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 6),
                CardAttribute::new(Attribute::SPEED, 8),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 7),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("8"),
            String::from("Justice"),
            String::from("Enya"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 7),
                CardAttribute::new(Attribute::SPEED, 8),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 7),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("9"),
            String::from("Emperor"),
            String::from("Hol Horse"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 8),
                CardAttribute::new(Attribute::SPEED, 8),
                CardAttribute::new(Attribute::RANGE, 9),
                CardAttribute::new(Attribute::STAMINA, 7),
                CardAttribute::new(Attribute::PRECISION, 8),
                CardAttribute::new(Attribute::DEVELOPMENT, 4),
            ],
        ),
        Card::new(
            String::from("10"),
            String::from("The Fool"),
            String::from("Iggy"),
            String::from("some desc"),
            false,
            String::from("image"),
            vec![
                CardAttribute::new(Attribute::POWER, 0),
                CardAttribute::new(Attribute::SPEED, 6),
                CardAttribute::new(Attribute::RANGE, 4),
                CardAttribute::new(Attribute::STAMINA, 10),
                CardAttribute::new(Attribute::PRECISION, 9),
                CardAttribute::new(Attribute::DEVELOPMENT, 9),
            ],
        ),
    ]
}
