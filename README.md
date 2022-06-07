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