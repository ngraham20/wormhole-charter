# wormhole-charter
Tracks EVE Online wormhole locations


- keep track of the connections, not the wormhole signatures
- connection has: 2 sides, each is a wormhole.
- connection has a lifetime and a time-to-live (ttl) 
- ttl should be given as a range, from the shortest possible lifespan to the longest

- vector of all wormholes
- maybe a connection only has indices???

- a wormhole is only a connection, it does not describe anything more than that. Therefore, what if we treat both wormhole signatures as actually just one wormhole?


- what would graphing the connections look like?
    - A tree walk maybe, root -> check connections and attach those -- doesn't really handle loops well though
    - some kind of state machine-like notation? -> display all systems in 


when discovering a wormhole signature,
what I know:
- the system I'm in
- the signature ID

when going through wormhole,
here's the info I already know:
- the system I came from
- the system I arrived in

here's info I _might_ know:
- the signature I used to travel

here's info I _don't_ know:
- the signature I arrived through