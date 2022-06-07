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
#[derive(Signature)]
struct CombatSite {}

#[signature]
#[derive(Signature)]
struct GasSite {}

#[signature]
#[derive(Signature)]
struct DataSite {}

#[signature]
#[derive(Signature)]
struct RelicSite {}

#[signature]
#[derive(Signature)]
struct OreSite {}

#[signature]
#[derive(Signature)]
struct Wormhole {}

use core::cell::RefCell;
struct WormholeLink<'a> {
    system_a: &'a StarSystem,
    system_b: &'a StarSystem,
    max_lifetime: usize,
}

type WormholeChain<'a> = HashMap<String, WormholeLink<'a>>;

struct StarSystem {
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
    
    let mut sol = StarSystem {
        cosmic_signatures: Vec::new()
    };

    let mut kerbol = StarSystem {
        cosmic_signatures: Vec::new()
    };

    let link_a = WormholeLink {
        system_a: &mut sol,
        system_b: &mut kerbol,
        max_lifetime: 64
    };

    link_a.system_a.cosmic_signatures = Vec::new();


    // either use interior mutablity (refcell) or make a hashset? of systems, and make the links
    // only store the keys to the global set of systems
}