mod feature;
mod state;

use std::{io::stdout, time::Duration};

use crossterm::{event::{KeyCode, poll, Event}, execute, style::{Print}, cursor::{MoveToPreviousLine, MoveToNextLine, MoveTo, DisableBlinking, Hide}, terminal::{Clear, ClearType}};
use feature::*;
use state::State;

fn main() {
    
    let mut features = create_features();
    let mut state = create_state();

    // setup terminal
    execute!(stdout(), Clear(ClearType::All), MoveTo(0,0), DisableBlinking, Hide);

    // first time render
    render(&features, &state);

    // simple game loop: process input, step, render
    loop {
        let key = wait(100);
        process_input(key, &features, &mut state);
        step(&mut features, &mut state);
        render(&features, &state);
    }
}

/// Create all features
fn create_features() -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = Vec::new();
    features.push(Box::new(counter::CounterFeature));
    features.push(Box::new(fight::FightFeature));
    features
}

/// Create initial state
fn create_state() -> State {
    State {
        count: 0,
        key: KeyCode::Null,
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

/// Process input
fn process_input(key: KeyCode, features: &Vec<Box<dyn Feature>>, state: &mut State) {
    
    if state.selected_feature.is_some() {
        match key {
            KeyCode::Char('q') => state.selected_feature = None,
            k => state.key = k,
        }
    } else {
        match key {
            KeyCode::Char('q') => std::process::exit(0),
            k => {
                state.selected_feature = {
                    features.iter().position(|f| f.get_key() == k)
                }
            },
        }
    }
}

/// Step the current selected feature
fn step(features: &mut Vec<Box<dyn Feature>>, state: &mut State) {
    if let Some(i) = state.selected_feature {
        let feature = &mut features[i];
        feature.update(state);
    }
}

/// Render the current selected feature, or the list of features
fn render(features: &Vec<Box<dyn Feature>>, state: &State) {
    if let Some(i) = state.selected_feature {
        let feature = &features[i];

        let str = format!("{}: {}", feature.get_name(), feature.render(state));
        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0), Print(str) );
        
    } else {
        let mut str = String::new();
        for feature in features {
            if let KeyCode::Char(k) = feature.get_key() {
                str.push_str(&format!("{} [{}] ", feature.get_name(), k));
            } else {
                str.push_str(&format!("{} [?]", feature.get_name()));
            }
        }

        execute!(stdout(), Clear(ClearType::All), MoveTo(0,0), Print(str));
    }
}

/// Wait for a key for a certain amount of time
fn wait(ms: u128) -> KeyCode {
    let mut input = KeyCode::Null;
    let now = std::time::Instant::now();

    loop {
        if now.elapsed().as_millis() >= ms {
            break input;
        }

        if poll(Duration::from_millis(10)).is_ok() {
            if let Ok(Event::Key(key)) = crossterm::event::read() {
                input = key.code;
            }
        };
    }
}