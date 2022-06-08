/// #######################################################
///  EVE Online Wormhole Tracker -- Signature Library
/// #######################################################

use wormhole_charter::*;
use std::collections::{HashMap};

fn main() {
    test_ui();
}

fn test_signatures() {
    let sig_a = CosmicSignature::CombatSite(Signature {
        id: "ABC-123".to_string(),
        name: "Perimeter Combat Site".to_string(),
        age: 0,
    });
    let sig_b = CosmicSignature::GasSite(Signature {
        id: "CBA-321".to_string(),
        name: "Bountiful Gas Site".to_string(),
        age: 0,
    });
    let sig_w = CosmicSignature::Wormhole(Wormhole {
        id: "BAC-213".to_string(),
        name: "Unstable Wormhole".to_string(),
        age: 0,
        wh_type: "K162".to_string(), 
    });
    
    let mut sol = StarSystem {
        name: "Sol".to_string(),
        cosmic_signatures: Vec::new()
    };

    let mut wormhole_chain: Vec<WormholeLink> = Vec::new();
    sol.discover_signature(sig_a.clone(), &mut wormhole_chain);
    sol.discover_signature(sig_b.clone(), &mut wormhole_chain);
    sol.discover_signature(sig_w.clone(), &mut wormhole_chain);

    let mut kerbol = StarSystem {
        name: "Kerbol".to_string(),
        cosmic_signatures: Vec::new()
    };

    kerbol.discover_signature(sig_a, &mut wormhole_chain);
    kerbol.discover_signature(sig_b, &mut wormhole_chain);
    kerbol.discover_signature(sig_w, &mut wormhole_chain);

    // let link_a = WormholeLink {
    //     system_a: "Sol".to_string(),
    //     system_b: "Kerbol".to_string(),
    //     wh_type: "C247".to_string(),
    //     max_lifetime: 16,
    //     age: 0
    // };
    // wormhole_chain.push(link_a);
    let mut star_systems: HashMap<String, StarSystem> = HashMap::new();
    star_systems.insert(sol.name.clone(), sol);
    star_systems.insert(kerbol.name.clone(), kerbol);

    for link in wormhole_chain {
        println!("Wormhole links {} ({}) to {} ({})", link.system_a.0, link.system_a.1, link.system_b.0, link.system_b.1);
        println!("{} has the following signatures: ", link.system_a.0);
        if let Some(system) = star_systems.get(&link.system_a.0) {
            for sig in system.cosmic_signatures.iter() {
                match sig {
                    CosmicSignature::CombatSite(s) |
                    CosmicSignature::DataSite(s) |
                    CosmicSignature::ReliceSite(s) |
                    CosmicSignature::GasSite(s) |
                    CosmicSignature::OreSite(s) => {println!("{} | {}", s.id, s.name)}
                    CosmicSignature::Wormhole(w) => {println!("{} | {}", w.id, w.name)}
                }
            }
        }
    }
}

use std::error::Error;
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
fn test_ui() -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        // let size = f.size();
        // let block = Block::default()
        //     .title("Block")
        //     .borders(Borders::ALL);
        // f.render_widget(block, size);
        ui(f);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

use tui::{
    backend::Backend,
    Frame,
};
fn ui<B: Backend>(f: &mut Frame<B>) {
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(f.size());
    let block = Block::default()
         .title("Block")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
         .title("Block 2")
         .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);
}
