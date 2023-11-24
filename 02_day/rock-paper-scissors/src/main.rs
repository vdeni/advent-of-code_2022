use std::collections::HashMap;
use std::path::Path;

fn main() {
    let current_file = Path::new(file!());

    let data_file = current_file
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("data/data.csv");
    let strategy_guide = load_strategy_guide(data_file);

    let total_player_score = calculate_total_score(strategy_guide);

    println!("The total player score is: {total_player_score}.");
}

fn load_strategy_guide<P>(file: P) -> HashMap<u32, (Shape, Shape)>
where
    P: AsRef<Path>,
{
    /*!
     * Load the given strategy guide from an external CSV file. Return a
     * dictionary holding an u32 round ID, and tuples of Shapes for the
     * opponent's and plyer's choices.
     */

    let mut strategy_guide: HashMap<u32, (Shape, Shape)> = HashMap::new();

    let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)
        .unwrap();

    let mut round_id = 1;

    for record in csv_reader.records() {
        if let Ok(entry) = record {
            if let (Some(opponent_move), Some(player_move)) = (entry.get(0), entry.get(1)) {
                let opponent_shape = match opponent_move {
                    "A" => Shape::Rock,
                    "B" => Shape::Paper,
                    "C" => Shape::Scissors,
                    _ => panic!("Unkown move found for opponent: {opponent_move}."),
                };
                let player_shape = match player_move {
                    "X" => Shape::Rock,
                    "Y" => Shape::Paper,
                    "Z" => Shape::Scissors,
                    _ => panic!("Unkown move found for player: {player_move}."),
                };

                strategy_guide.insert(round_id, (opponent_shape, player_shape));
                round_id += 1;
            }
        }
    }

    return strategy_guide;
}

fn calculate_total_score(strategy_guide: HashMap<u32, (Shape, Shape)>) -> u64 {
    /*!
     * Given a provided strategy, calculate the total player score for all rounds.
     */
    let total_shape_score = get_total_shape_value(&strategy_guide);
    let total_contest_score = get_total_contest_score(&strategy_guide);

    return total_shape_score + total_contest_score;
}

fn get_total_contest_score(strategy_guide: &HashMap<u32, (Shape, Shape)>) -> u64 {
    /*!
     * Determines the total score of all the opponent vs player contests.
     */

    let round_outcomes: Vec<RoundOutcome> = strategy_guide
        .values()
        .map(determine_round_winner)
        .collect();

    let total_contest_score = round_outcomes.iter().map(get_outcome_value).sum::<u64>();

    return total_contest_score;
}

fn determine_round_winner(round_choice_pair: &(Shape, Shape)) -> RoundOutcome {
    /*!
     * Takes a pair of RoundOutcome variants representing the opponent's shape
     * choice and the player's shape choice, and returns an enum telling whether
     * the player won, the opponent won, or there was a tie.
     */

    let outcome = match round_choice_pair {
        (Shape::Rock, Shape::Paper) => RoundOutcome::PlayerWon,
        (Shape::Rock, Shape::Scissors) => RoundOutcome::OpponentWon,
        (Shape::Paper, Shape::Rock) => RoundOutcome::OpponentWon,
        (Shape::Paper, Shape::Scissors) => RoundOutcome::PlayerWon,
        (Shape::Scissors, Shape::Rock) => RoundOutcome::PlayerWon,
        (Shape::Scissors, Shape::Paper) => RoundOutcome::OpponentWon,
        _ => RoundOutcome::Tie,
    };

    return outcome;
}

fn get_outcome_value(round_outcome: &RoundOutcome) -> u64 {
    /*!
     * Convert each RoundOutcome to a numeric value,
     */

    let value = match round_outcome {
        RoundOutcome::PlayerWon => 6,
        RoundOutcome::OpponentWon => 0,
        RoundOutcome::Tie => 3,
    };

    return value;
}

fn get_total_shape_value(strategy_guide: &HashMap<u32, (Shape, Shape)>) -> u64 {
    /*!
     * Takes all the shapes the player's played in the strategy guide and
     * retunrs their total value. This is added to the scores obtained from
     * the results of each individual round, giving the total tournament score.
     */

    let total_shape_value: u64 = strategy_guide
        .values()
        .map(|shape_tuple| get_single_shape_value(&shape_tuple.1))
        .sum();

    return total_shape_value;
}

fn get_single_shape_value(shape: &Shape) -> u64 {
    /*!
     * Fetch the value of a single given shape
     */
    let value = match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    return value;
}

enum RoundOutcome {
    PlayerWon,
    OpponentWon,
    Tie,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}
