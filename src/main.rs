mod feature;
mod state;

use console::{Term, Key};

use feature::*;
use state::State;

fn main() {
    
    let mut features = create_features();
    let mut state = create_state();

    let term = Term::stdout();

    // first time render
    render(&features, &state, &term);

    loop {
        process_input(&features, &mut state, &term);
        step(&mut features, &mut state);
        render(&features, &state, &term);
    }
}

fn create_features() -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = Vec::new();
    features.push(Box::new(counter::CounterFeature));
    features.push(Box::new(fight::FightFeature));
    features
}

fn create_state() -> State {
    State {
        count: 0,
        key: Key::Unknown,
        selected_feature: None,

        fight: {
            let player = fight::Player {
                attack: 2,
                defense: 0,
                health: 10,
                max_health: 10,
            };
            let enemy = fight::Enemy {
                attack: 1,
                defense: 0,
                health: 10,
                max_health: 10,
            };
            fight::FightData { player, enemy }
        }
    }
}

fn process_input(features: &Vec<Box<dyn Feature>>, state: &mut State, term: &Term) {
    let input = term.read_key().unwrap_or(Key::Unknown);

    if state.selected_feature.is_some() {
        match input {
            Key::Char('q') => state.selected_feature = None,
            k => state.key = k,
        }
    } else {
        match input {
            Key::Char('q') => std::process::exit(0),
            k => {
                state.selected_feature = {
                    features.iter().position(|f| f.get_key() == k)
                }
            },
        }
    }
}

fn step(features: &mut Vec<Box<dyn Feature>>, state: &mut State) {
    if let Some(i) = state.selected_feature {
        let feature = &mut features[i];
        feature.update(state);
    }
}

fn render(features: &Vec<Box<dyn Feature>>, state: &State, term: &Term) {
    if let Some(i) = state.selected_feature {
        let feature = &features[i];

        let str = format!("{}: {}", feature.get_name(), feature.render(state));
        term.write_line(&str);
        
    } else {
        let mut str = String::new();
        for feature in features {
            if let Key::Char(k) = feature.get_key() {
                str.push_str(&format!("{} [{}] ", feature.get_name(), k));
            } else {
                str.push_str(&format!("{} [?]", feature.get_name()));
            }
        }

        term.clear_last_lines(1);
        term.write_line(&str);
    }
}
