mod feature;
mod state;

use std::{io::{stdout, Write}, time::Duration};

use crossterm::{event::{KeyCode, poll, Event}, execute, style::{Print, StyledContent, Stylize}, cursor::{MoveToNextLine, MoveTo, DisableBlinking, Hide}, terminal::{Clear, ClearType}, queue};
use feature::*;
use state::State;

fn main() {
    
    let mut features = create_features();
    let mut state = create_state();

    let ms_step: u32 = 100;

    // setup terminal
    execute!(stdout(), 
        Clear(ClearType::All), 
        MoveTo(0,0), 
        DisableBlinking, 
        Hide
    ).expect("Failed to setup terminal");

    // first time render
    render(&features, &state);

    // simple game loop: process input, step, render
    loop {
        let key = wait_key(ms_step.into());
        process_input(key, &features, &mut state);
        step(ms_step as f32 / 1000.0, &mut features, &mut state);
        render(&features, &state);
    }
}

/// Create all features
fn create_features() -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = Vec::new();
    features.push(Box::new(exit::ExitFeature));
    features.push(Box::new(counter::CounterFeature));
    features.push(Box::new(fight::FightFeature::default()));
    features
}

/// Create initial state
fn create_state() -> State {
    State {
        count: 0,
        key: KeyCode::Null,
        selected_feature: None,

        fight: fight::FightData::default(),
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
            k => {
                state.selected_feature = {
                    features.iter().position(|f| f.get_key() == k)
                }
            },
        }
    }
}

/// Step the current selected feature
fn step(ms_step: f32, features: &mut Vec<Box<dyn Feature>>, state: &mut State) {
    if let Some(i) = state.selected_feature {
        let feature = &mut features[i];
        feature.update(ms_step, state);
    }
}

/// Render the current selected feature, or the list of features
fn render(features: &Vec<Box<dyn Feature>>, state: &State) {

    let mut stdout = stdout();

    // render the selected feature
    if let Some(i) = state.selected_feature {
        let feature = &features[i];

        queue!(stdout, 
            Clear(ClearType::All), 
            MoveTo(0,0), 
            Print(feature.get_name()),
        ).expect("Failed to render");

        render_keys(feature.get_inputs()).iter().for_each(|s| {
            queue!(stdout, Print(s)).expect("Failed to render");
        });

        queue!(stdout,
            MoveToNextLine(1),
            Print(feature.render(state)),
        ).expect("Failed to render");
    
    // or render the list of features
    } else {
        let mut str = String::new();
        for feature in features {
            str.push_str(&format!("[{}]{} ", get_string(feature.get_key()), feature.get_name()));
        }

        queue!(stdout, 
            Clear(ClearType::All), 
            MoveTo(0,0), 
            Print(str)
        ).expect("Failed to render");
    }

    stdout.flush().expect("Failed to render");
}

/// Render a list of keys into a list of styled strings
fn render_keys(keys: Vec<(KeyCode, StyledContent<String>)>) -> Vec<StyledContent<String>> {
    let mut str: Vec<StyledContent<String>> = Vec::new();
    for (key, text) in keys {
        str.push(format!(" [{}]", get_string(key)).stylize());
        str.push(text);
    }
    str
}

/// Wait for a key for a certain amount of time
fn wait_key(ms: u128) -> KeyCode {
    let mut input = KeyCode::Null;
    let now = std::time::Instant::now();

    loop {
        if now.elapsed().as_millis() >= ms {
            break input;
        }

        if let Ok(true) = poll(Duration::from_millis(10)) {
            if let Ok(Event::Key(key)) = crossterm::event::read() {
                input = key.code;
            }
        };
    }
}

/// Get a string representation of a key
fn get_string(key: KeyCode) -> String {
    match key {
        KeyCode::Char(c) => format!("{}", c),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Left => "<-".to_string(),
        KeyCode::Right => "->".to_string(),
        _ => "?".to_string(),
    }
}
