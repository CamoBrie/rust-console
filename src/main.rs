mod feature;
mod state;

use console::Term;

use feature::*;
use state::State;

fn main() {
    
    let mut features = create_features();
    let mut state = create_state();

    loop {
        process_input(&mut state);
        step(&mut features, &mut state);
        render(&features, &state);
    }

}

fn create_features() -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = Vec::new();
    features.push(Box::new(counter::CounterFeature));
    features
}

fn create_state() -> State {
    State {
        count: 0,
        key: ' ',
    }
}

fn process_input(state: &mut State) {
    let input: char = Term::stdout().read_char().unwrap_or(' ');
    match input {
        'q' => std::process::exit(0),
        k => state.key = k,
    }
}

fn step(features: &mut Vec<Box<dyn Feature>>, state: &mut State) {
    for feature in features {
        feature.update(state);
    }
}

fn render(features: &Vec<Box<dyn Feature>>, state: &State) {
    for feature in features {
        println!("{}", feature.render(state));
    }
}
