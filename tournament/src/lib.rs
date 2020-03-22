use std::collections::HashMap;

#[derive(Default, Debug)]
struct Team {
    mp: u8,
    wins: u8,
    draws: u8,
    lost: u8,
    points: u8,
}

impl Team {
    pub fn add_win(&mut self) {
        self.mp += 1;
        self.wins += 1;
        self.points += 3;
    }

    pub fn add_draw(&mut self) {
        self.mp += 1;
        self.points += 1;
        self.draws += 1;
    }

    pub fn add_loss(&mut self) {
        self.mp += 1;
        self.lost += 1;
    }
}

fn build_table(results: &str) -> HashMap<&str, Team> {
    let mut table: HashMap<&str, Team> = HashMap::new();

    results.split('\n').for_each(|line| {
        let match_info = line.split(';').collect::<Vec<&str>>();
        if let [primary, secondary, result] = match_info.as_slice() {
            match *result {
                "win" => {
                    table.entry(&primary).or_default().add_win();
                    table.entry(&secondary).or_default().add_loss();
                }
                "draw" => {
                    table.entry(&primary).or_default().add_draw();
                    table.entry(&secondary).or_default().add_draw();
                }
                "loss" => {
                    table.entry(&primary).or_default().add_loss();
                    table.entry(&secondary).or_default().add_win();
                }
                _ => unreachable!(),
            };
        }
    });

    table
}

fn format_table(results: HashMap<&str, Team>) -> String {
    let mut lines: Vec<String> = Vec::new();
    lines.push("Team                           | MP |  W |  D |  L |  P".to_string());

    let mut v: Vec<(String, Team)> = Vec::new();
    results
        .into_iter()
        .for_each(|(n, t)| v.push((n.to_string(), t)));
    v.sort_by(|(n1, ts1), (n2, ts2)| (ts2.points, n1).cmp(&(ts1.points, n2)));

    v.iter().for_each(|(n, r)| {
        lines.push(format!(
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            n.to_string(),
            r.mp,
            r.wins,
            r.draws,
            r.lost,
            r.points
        ));
    });

    println!("{}", lines.join("\n"));

    lines.join("\n")
}

pub fn tally(match_results: &str) -> String {
    format_table(build_table(match_results))
}
