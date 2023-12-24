use strategies::*;
mod strategies;
use ::clap::Parser;
use plotters::prelude::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Print individual runs
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
    #[arg(short, long, default_value = "./output.png")]
    output_file: String,
    /// Value between 0 and 1. Flips action to create noise
    #[arg(short, long, default_value_t = 0.0)]
    noise: f64,
}
struct Runner {
    players: Vec<(Box<dyn Strategy>, usize)>,
    runs: Vec<(String, String, usize, usize)>,
    run_length: usize,
    iterations: usize,
    noise: f64,
}

impl Runner {
    fn new(players: Vec<Box<dyn Strategy>>, noise: f64) -> Self {
        let players = players.into_iter().map(|x| (x, 0)).collect();
        Runner {
            players,
            runs: Vec::new(),
            run_length: 200,
            iterations: 5,
            noise,
        }
    }

    fn match_run(&mut self, left: usize, right: usize) {
        let mut left_actions: Vec<Action> = Vec::new();
        let mut right_actions: Vec<Action> = Vec::new();
        let mut left_score = 0;
        let mut right_score = 0;

        for _ in 0..self.run_length {
            // PLayers decide their actions
            let left_action = self.players[left]
                .0
                .next_move(&right_actions, &left_actions);
            let right_action = self.players[right]
                .0
                .next_move(&left_actions, &right_actions);
            // decide results
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
            // Introduce noise in communication
            right_actions.push(right_action.noise(self.noise));
            left_actions.push(left_action.noise(self.noise));
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

fn generate_chart(
    scores: Vec<(String, usize)>,
    output_file: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&output_file, (1280, 960)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(80)
        .margin(5)
        .caption("Prisoners", ("sans-serif", 50.0))
        .build_cartesian_2d(0..scores[0].1, (0..(scores.len() - 1)).into_segmented())?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Players")
        .x_desc("Scores")
        .axis_desc_style(("sans-serif", 15))
        .y_labels(scores.len())
        .y_label_formatter(&|y| match y {
            SegmentValue::CenterOf(v) => scores[*v as usize].0.clone(),
            _ => "UNK".to_string(),
        })
        .draw()?;

    chart.draw_series(
        Histogram::horizontal(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(scores.iter().rev().enumerate().map(|(i, x)| (i, x.1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("\nResult has been saved to {}", output_file);
    Ok(())
}
fn main() {
    let args = Args::parse();
    let players: Vec<Box<dyn Strategy>> = vec![
        Box::new(Naive),
        Box::new(Exploiter),
        Box::new(Random),
        Box::new(Grudger),
        Box::new(Reluctant),
        Box::new(Detective),
        Box::new(TitForTat),
        Box::new(FlipFlop),
        Box::new(TitForTatN(2)),
        Box::new(TitForTatN(3)),
    ];

    let mut runner = Runner::new(players, args.noise);
    runner.run();
    if args.verbose {
        for (lname, rname, lscore, rscore) in &runner.runs {
            println!("{} vs {}: {} - {}", lname, rname, lscore, rscore);
        }
    }
    let mut scores = runner.scores();
    scores.sort_by_key(|x| x.1);
    scores.reverse();
    generate_chart(scores, args.output_file).unwrap();
}
