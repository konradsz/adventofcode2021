enum Turn {
    Player1,
    Player2,
}

fn main() {
    let mut player1_pos = 4;
    let mut player1_score = 0;
    let mut player2_pos = 3;
    let mut player2_score = 0;
    let mut turn = Turn::Player1;

    let rounds = (1..=100)
        .cycle()
        .step_by(3)
        .take_while(|side| {
            let side2 = if (side + 1) > 100 {
                (side + 1) % 100
            } else {
                side + 1
            };
            let side3 = if (side + 2) > 100 {
                (side + 2) % 100
            } else {
                side + 2
            };
            let total = side + side2 + side3;

            match turn {
                Turn::Player1 => {
                    player1_pos = if (player1_pos + total) % 10 == 0 {
                        10
                    } else {
                        (player1_pos + total) % 10
                    };

                    player1_score += player1_pos;
                    turn = Turn::Player2;
                }
                Turn::Player2 => {
                    player2_pos = if (player2_pos + total) % 10 == 0 {
                        10
                    } else {
                        (player2_pos + total) % 10
                    };

                    player2_score += player2_pos;
                    turn = Turn::Player1;
                }
            }

            player1_score < 1000 && player2_score < 1000
        })
        .count()
        + 1;

    assert_eq!(
        rounds * 3 * std::cmp::min(player1_score, player2_score),
        734820
    )
}
