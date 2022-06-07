/// #######################################################
///  EVE Online Wormhole Tracker
/// 

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::collections::{HashMap};
type WormHoleType = HashMap<String, Wormhole>;

use wormhole_charter::{Signature};
use wormhole_charter_derive::{signature, Signature};

#[signature]
#[derive(Signature, Clone)]
struct CombatSite {}

#[signature]
#[derive(Signature, Clone)]
struct GasSite {}

#[signature]
#[derive(Signature, Clone)]
struct DataSite {}

#[signature]
#[derive(Signature, Clone)]
struct RelicSite {}

#[signature]
#[derive(Signature, Clone)]
struct OreSite {}

#[signature]
#[derive(Signature, Clone)]
struct Wormhole {}

use core::cell::RefCell;
struct WormholeLink {
    system_a: String,
    system_b: String,
    max_lifetime: usize,
}

struct StarSystem {
    name: String,
    cosmic_signatures: Vec<Box<dyn Signature>>
}



//                      [ W0 ]
//                
//             [ W1 ]   [ W2 ]   [ W3 ]

// fn load_whdata() -> Result<WormHoleType, Box<dyn Error>> {
//     let file = File::open("wormholes.json").expect("oops");
//     let reader = BufReader::new(file);

//     let jdata: WormHoleType = serde_json::from_reader(reader)?;
//     Ok(jdata)
// }

fn main() {

    let sig_a = CombatSite {
        id: "ABC-123".to_string(),
        name: "Perimeter Combat Site".to_string(),
        age: 0,
    };
    let sig_b = GasSite {
        id: "CBA-321".to_string(),
        name: "Bountiful Gas Site".to_string(),
        age: 0,
    };
    
    let mut sol = StarSystem {
        name: "Sol".to_string(),
        cosmic_signatures: Vec::new()
    };

    sol.cosmic_signatures.push(Box::new(sig_a.clone()));
    sol.cosmic_signatures.push(Box::new(sig_b.clone()));

    let mut kerbol = StarSystem {
        name: "Kerbol".to_string(),
        cosmic_signatures: Vec::new()
    };

    kerbol.cosmic_signatures.push(Box::new(sig_a));
    kerbol.cosmic_signatures.push(Box::new(sig_b));

    let link_a = WormholeLink {
        system_a: "Sol".to_string(),
        system_b: "Kerbol".to_string(),
        max_lifetime: 64
    };

    let mut wormhole_chain: Vec<WormholeLink> = Vec::new();
    wormhole_chain.push(link_a);
    let mut star_systems: HashMap<String, StarSystem> = HashMap::new();
    star_systems.insert(sol.name.clone(), sol);
    star_systems.insert(kerbol.name.clone(), kerbol);

    for link in wormhole_chain {
        println!("Wormhole links {} to {}", link.system_a, link.system_b);
        println!("{} has the following signatures: ", link.system_a);
        if let Some(system) = star_systems.get(&link.system_a) {
            for sig in system.cosmic_signatures.iter() {
                println!("{}", sig.name());
            }
        }
    }
}