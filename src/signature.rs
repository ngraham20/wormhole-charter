/// #######################################################
///  EVE Online Wormhole Tracker -- Signature Library
/// #######################################################

/// Cosmic Signature
/// 
/// All Cosmic Signatures are Signatures, except for Wormhole, which is its own type
#[derive(Clone)]
pub enum CosmicSignature {
    CombatSite(Signature),
    DataSite(Signature),
    ReliceSite(Signature),
    GasSite(Signature),
    OreSite(Signature),
    Wormhole(Wormhole)
}

#[derive(Clone)]
pub struct Signature {
    pub id: String,
    pub name: String,
    pub age: usize,
}

#[derive(Clone)]
pub struct Wormhole {
    pub id: String,
    pub name: String,
    pub age: usize,
    pub wh_type: String
}
impl Wormhole {
    fn increment_age(&mut self) {
        self.age += 1;
    }
}

/// A link in the Wormhole Chain
/// 
/// A link keeps track of the systems on either side of it and attempts to
/// automate some of the tedious parts of tracking where the holes lead to
/// and how long they'll be stable for.
/// 
/// TODO: Add functionality to assist in rolling a new hole via Praxis destabalization.
pub struct WormholeLink {
    pub system_a: (String, String),
    pub system_b: (String, String),
    pub wh_type: String,
    pub max_lifetime: usize,
    pub age: usize
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
        todo!();
    }

    fn establish(system: &StarSystem, sig: &Wormhole) -> Self {
        WormholeLink {
            system_a: (system.name.clone(), sig.id.clone()),
            system_b: ("Unknown Star System".to_string(), "???-???".to_string()),
            wh_type: "????".to_string(),
            max_lifetime: 969,
            age: 0
        }
    }

    fn traverse(&mut self, system: &StarSystem) {

    }

    fn update_source_sig(&mut self, sig: &CosmicSignature) {

    }

    fn complete(&mut self, sig: &CosmicSignature) {
        
    }
}

pub struct StarSystem {
    pub name: String,
    pub cosmic_signatures: Vec<CosmicSignature>
}
impl StarSystem {
    pub fn new() -> Self {
        StarSystem {
            name: "Unknown System".to_string(),
            cosmic_signatures: Vec::new()
        }
    }
    pub fn named(name: String) -> Self {
        StarSystem {
            name: name,
            cosmic_signatures: Vec::new()
        }
    }

    /// Discovers a new cosmic signature in the current system.
    /// 
    /// - If the signature is a wormhole, create a new link in the wormhole chain
    ///     - Call on the wormhole type file to automatically fill the maximum lifetime stat
    /// - If the signature is anything else, don't worry about it
    /// - Regardless, add the signature to the list of signatures in the system
    pub fn discover_signature(&mut self, sig: CosmicSignature, wh_chain: &mut Vec<WormholeLink>) {
       match &sig {
           CosmicSignature::Wormhole(w) => {
                // TODO call wormhole information file in here to automate maximum lifetime in minutes
                // wh_chain.push(WormholeLink {
                //     system_a: self.name.clone(),
                //     system_a_sig: w.id.clone(),
                //     system_b: "Unknown Star System".to_string(),
                //     system_b_sig: "???-???".to_string(),
                //     wh_type: w.wh_type.clone(),
                //     max_lifetime: 42,
                //     age: 0
                // })
                wh_chain.push(WormholeLink::establish(self, w));
            }
            _ => {}
       }
       self.cosmic_signatures.push(sig);
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