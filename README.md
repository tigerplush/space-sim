# space-sim

Create new crates with `cargo new --lib ./crates/new_crate --vcs none`

## Features

### realistic oxygen simulation
* a player should consume oxygen and produce carbon dioxide
* in case of a leak, the pressure in the ship should fall
* is the pressure too low, the player faints and dies
* life support should take carbon dioxide and produce oxygen

### realistic thermal simulation
* devices produce heat
* the ship looses heat via its surface
* the life support can cool or heat the ship

### ship parts
* every ship part should be exchangeable
* every ship part can be disassembled
* ship parts can be connected to each other via different methods, possible methods are:
    * wireless
    * bus-system
    * ethernet
    * power-cable
    * plasma-conduit
    * waveguide
* all parts need power, some can be powered by energy, others by some kind of material (fuel etc.)

#### examples
1. The ship has a tank of fuel. A reactor takes fuel, creates heat and power. The power can be moved to the FTL drive via plasma-conduit.
In the cockpit you start the FTL by calculating the jump, which takes a few seconds and consumes power, and then you send the calculations to the drive via a controller (which also takes power).

