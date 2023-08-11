mod feature;
mod message;
mod state;
mod util;

use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, Event, KeyCode},
    execute, queue,
    style::{Print, Stylize},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use feature::{counter, exit, fight, inventory, shop, Feature};
use state::State;
use util::conv::get_string;

use crate::util::commands::{Divider, PrintAll, PrintAllLines};

fn main() -> std::io::Result<()> {
    enable_raw_mode()?;

    let mut features = create_features();
    let mut state = create_state();

    let mut message_manager = message::MessageManager::default();

    let ms_step: u32 = 100;

    // setup terminal
    execute!(
        stdout(),
        EnterAlternateScreen,
        Clear(ClearType::All),
        MoveTo(0, 0),
        DisableBlinking,
        Hide,
    )?;

    // render hello message
    message_manager.add_message(message::Message {
        text: "Welcome to the game! First, go into the Counter feature.  You leave a feature with [q].".to_string().bold(),
        location: message::TextLocation::Center,
        duration: 5.0,
    });

    // first time render
    render(&features, &state, &mut message_manager);

    // simple game loop: process input, step, render
    loop {
        let key = wait_key(ms_step.into());
        process_input(key, &features, &mut state);
        step(
            ms_step as f32 / 1000.0,
            &mut features,
            &mut state,
            &mut message_manager,
        );
        render(&features, &state, &mut message_manager);

        if state.quit {
            break;
        }
    }

    disable_raw_mode()?;

    // cleanup terminal
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        EnableBlinking,
        Show,
        LeaveAlternateScreen
    )?;
    Ok(())
}

/// Create all features
fn create_features() -> Vec<Box<dyn Feature>> {
    let mut features: Vec<Box<dyn Feature>> = Vec::new();
    features.push(Box::new(exit::ExitFeature));
    features.push(Box::new(counter::CounterFeature));
    features.push(Box::new(fight::FightFeature::default()));
    features.push(Box::new(inventory::InventoryFeature::default()));
    features.push(Box::new(shop::ShopFeature));
    features
}

/// Create initial state
fn create_state() -> State {
    State {
        key: KeyCode::Null,
        selected_feature: None,
        quit: false,

        count: 0,
        fight: fight::FightData::default(),
        inventory: inventory::Inventory::default(),
        upgrades: shop::Upgrades::default(),
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
                state.selected_feature = features.iter().position(|f| {
                    f.is_unlocked(state)
                        && state.count >= f.get_info().unlock_count
                        && f.get_info().key == k
                })
            }
        }
    }
}

/// Step the current selected feature
fn step(
    delta: f32,
    features: &mut Vec<Box<dyn Feature>>,
    state: &mut State,
    message: &mut message::MessageManager,
) {
    if let Some(i) = state.selected_feature {
        let feature = &mut features[i];
        feature.update(delta, state, message);
    }

    message.update(state.key, delta);
}

/// Render the current selected feature, or the list of features
fn render(features: &Vec<Box<dyn Feature>>, state: &State, message: &mut message::MessageManager) {
    let mut stdout = stdout();

    // render the selected feature
    if let Some(i) = state.selected_feature {
        let feature = &features[i];

        queue!(
            stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(feature.get_info().name),
            PrintAll(feature.get_top_bar(state)),
            MoveToNextLine(1),
            Divider('='),
            PrintAllLines(feature.render(state, features))
        )
        .expect("Failed to render");

    // or render the list of features
    } else {
        let mut str = String::new();
        for feature in features {
            let info = feature.get_info();

            if feature.is_unlocked(state) && state.count >= info.unlock_count {
                str.push_str(&format!("[{}]{} ", get_string(info.key), info.name));
            } else {
                str.push_str(&format!("{} ", info.name.crossed_out()));
            }
        }

        queue!(stdout, Clear(ClearType::All), MoveTo(0, 0), Print(str)).expect("Failed to render");
    }

    message.render_one();

    stdout.flush().expect("Failed to render");
}

/// Wait for a key for a certain amount of time
fn wait_key(ms: u128) -> KeyCode {
    let mut input = KeyCode::Null;
    let now = std::time::Instant::now();

    loop {
        if now.elapsed().as_millis() >= ms {
            break input;
        }

        if let Ok(true) = poll(Duration::from_millis(0)) {
            if let Ok(Event::Key(key)) = crossterm::event::read() {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    input = key.code;
                }
            }
        };
    }
}
