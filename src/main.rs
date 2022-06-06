/// #######################################################
///  EVE Online Wormhole Tracker
/// 

// struct Wormhole {
//     id: [u8;6],
//     whtype: 
// }

// struct Signature {
//     id: [u8;6],
//     name: String,
//     sigtype: SigType,
// }
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
#[derive(Signature, Serialize, Deserialize, Debug)]
struct Wormhole {
    wormhole_type: String,
    destination: String,
    ttl: usize,
}

fn load_whdata() -> Result<WormHoleType, Box<dyn Error>> {
    let file = File::open("wormholes.json").expect("oops");
    let reader = BufReader::new(file);

    let jdata: WormHoleType = serde_json::from_reader(reader)?;
    Ok(jdata)
}

fn main() {
    
    let w = load_whdata();
    println!("{:?}", w);
}