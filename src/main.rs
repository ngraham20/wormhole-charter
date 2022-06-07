/// #######################################################
///  EVE Online Wormhole Tracker
/// 

use serde::{Serialize, Deserialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::collections::{HashMap};

#[derive(Clone)]
enum CosmicSignature {
    CombatSite(Signature),
    DataSite(Signature),
    ReliceSite(Signature),
    GasSite(Signature),
    OreSite(Signature),
    Wormhole(Wormhole)
}

#[derive(Clone)]
struct Signature {
    id: String,
    name: String,
    age: usize,
}


// #[signature]
#[derive(Clone)]
struct Wormhole {
    id: String,
    name: String,
    age: usize,
    wh_type: String
}
impl Wormhole {
    fn increment_age(&mut self) {
        self.age += 1;
    }
}

struct WormholeLink {
    system_a: String,
    system_b: String,
    wh_type: String,
    max_lifetime: usize,
    age: usize
}
impl WormholeLink {
    /// Calculates the Time to Live for the wormhole.
    /// 
    /// This is based on the maximum lifetime, the age, and the discovery of EoL status.
    /// 
    /// ### EXAMPLE
    /// - Suppose the hole's maximum lifetime is 16 hours
    /// - Suppose it was discovered 10 hours into its lifetime
    /// - age: 0, max_lifetime: 16, est. ttl: 16, true ttl: 6
    /// When 3 hours pass, someone notices the hole is EoL  
    /// Can we calculate the minimum possible ttl? The true ttl is 3.
    /// Max est. ttl is 4, the age since discovery is 3.
    /// The EoL status must have happened at _worst_, just after discovery.
    /// Therefore, if the age is less than 4, then the minimum ttl is 4-age.
    /// If the age is higher than 4, assume critical ttl.
    fn calculate_time_to_live(&self) -> (usize, usize) {
        todo!();
    }

    fn increment_age(&mut self) {
        self.age += 1;
    }

    /// Merges two links, accessed from the chain by index.
    /// 
    /// The first link's side_b gets updated to the second link's side_a.
    /// Link b is removed from the chain, as link a now accounts for both.
    fn merge_links(ind_a: usize, ind_b: usize, chain: &mut Vec<WormholeLink>) {
        let link_b = chain.remove(ind_b);
        let link_a = chain.get_mut(ind_a).unwrap();
        link_a.system_b = link_b.system_a;
    }
}

struct StarSystem {
    name: String,
    cosmic_signatures: Vec<CosmicSignature>
}
impl StarSystem {
    fn new() -> Self {
        StarSystem {
            name: "Unknown System".to_string(),
            cosmic_signatures: Vec::new()
        }
    }
    fn named(name: String) -> Self {
        StarSystem {
            name: name,
            cosmic_signatures: Vec::new()
        }
    }

    fn discover_signature(&mut self, sig: CosmicSignature) {
        self.cosmic_signatures.push(sig);
    }

    /// Discovers a new wormhole cosmic signature in this system
    /// - Check the wormhole information file and determine maximum lifetime based on wormhole type
    fn discover_wormhole(&mut self, wormhole: Wormhole, chain: &mut Vec<WormholeLink>) {
        // TODO call wormhole information file in here to automate maximum lifetime in minutes
        chain.push(WormholeLink {
            system_a: self.name.clone(),
            system_b: "Unknown Star System".to_string(),
            wh_type: wormhole.wh_type.clone(),
            max_lifetime: 42,
            age: 0
        });
        self.cosmic_signatures.push(CosmicSignature::Wormhole(wormhole));
    }

    fn discover_signatures(&mut self, sigs: Vec<CosmicSignature>) {
        for sig in sigs.iter() {
            // match sig {
                
            // }
        }
    }
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

    sol.discover_signature(sig_a.clone());
    sol.discover_signature(sig_b.clone());
    sol.discover_signature(sig_w.clone());

    let mut kerbol = StarSystem {
        name: "Kerbol".to_string(),
        cosmic_signatures: Vec::new()
    };

    kerbol.discover_signature(sig_a);
    kerbol.discover_signature(sig_b);
    kerbol.discover_signature(sig_w);

    let link_a = WormholeLink {
        system_a: "Sol".to_string(),
        system_b: "Kerbol".to_string(),
        wh_type: "C247".to_string(),
        max_lifetime: 16,
        age: 0
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
                match sig {
                    CosmicSignature::CombatSite(s) |
                    CosmicSignature::DataSite(s) |
                    CosmicSignature::ReliceSite(s) |
                    CosmicSignature::GasSite(s) |
                    CosmicSignature::OreSite(s) => {println!("{}", s.name)}
                    CosmicSignature::Wormhole(w) => {println!("{}", w.name)}
                }
            }
        }
    }
}