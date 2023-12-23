use strategies::{
    Action,
    Exploiter,
    FlipFlop,
    Grudger,
    Naive,
    Random,
    Reluctant,
    Strategy,
    TitForTat,
    TitForTatN,
};
mod strategies;

use ::clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print individual runs
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}
struct Runner {
    players: Vec<(Box<dyn Strategy>, usize)>,
    runs: Vec<(String, String, usize, usize)>,
    run_length: usize,
    iterations: usize,
}

impl Runner {
    fn new(players: Vec<Box<dyn Strategy>>) -> Self {
        let players = players.into_iter().map(|x| (x, 0)).collect();
        Runner {
            players,
            runs: Vec::new(),
            run_length: 200,
            iterations: 5,
        }
    }

    fn match_run(&mut self, left: usize, right: usize) {
        let mut left_actions: Vec<Action> = Vec::new();
        let mut right_actions: Vec<Action> = Vec::new();
        let mut left_score = 0;
        let mut right_score = 0;

        for _ in 0..self.run_length {
            let left_action = self.players[left]
                .0
                .next_move(&right_actions, &left_actions);
            let right_action = self.players[right]
                .0
                .next_move(&left_actions, &right_actions);
            match (&left_action, &right_action) {
                (Action::Cooperate, Action::Cooperate) => {
                    left_score += 3;
                    right_score += 3;
                }
                (Action::Cooperate, Action::Defect) => {
                    right_score += 5;
                }
                (Action::Defect, Action::Cooperate) => {
                    left_score += 5;
                }
                (Action::Defect, Action::Defect) => {
                    left_score += 1;
                    right_score += 1;
                }
            }
            right_actions.push(right_action);
            left_actions.push(left_action);
        }

        self.runs.push((
            self.players[left].0.name(),
            self.players[right].0.name(),
            left_score,
            right_score,
        ));

        self.players[left].1 += left_score;
        self.players[right].1 += right_score;
    }

    fn run(&mut self) {
        let total = self.players.len();
        for _ in 0..self.iterations {
            for i in 0..total {
                for ii in (i + 1)..total {
                    self.match_run(i, ii);
                }
            }
        }
    }

    fn scores(&self) -> Vec<(String, usize)> {
        self.players
            .iter()
            .map(move |x| (x.0.name(), x.1))
            .collect()
    }
}

fn main() {
    let args = Args::parse();
    let players: Vec<Box<dyn Strategy>> = vec![
        Box::new(Naive),
        Box::new(Exploiter),
        Box::new(Random),
        Box::new(Grudger),
        Box::new(Reluctant),
        Box::new(TitForTat),
        Box::new(FlipFlop),
        Box::new(TitForTatN(2)),
        Box::new(TitForTatN(3)),
    ];

    let mut runner = Runner::new(players);
    runner.run();
    if args.verbose {
        for (lname, rname, lscore, rscore) in &runner.runs {
            println!("{} vs {}: {} - {}", lname, rname, lscore, rscore);
        }
    }
    let mut scores = runner.scores();
    scores.sort_by_key(|x| x.1);
    scores.reverse();

    println!("\n\nFinal scores:");
    for score in scores {
        println!("{} - {}", score.0, score.1);
    }
}
