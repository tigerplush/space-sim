# space-sim

Create new crates with `cargo new --lib ./crates/new_crate --vcs none`

## Requirements
* A player owns a space ship
* Each part of the space ship must be replacable
* Each part can wear down over time or because of usage
* There should be tanks that can either house liquids or gases
* There should be hoses and pipes that can transport liquids and gases
* There should be generators, that take in some kind of fuel (liquid, gaseous or solid) and turn it into electricity, heat and exhaust gas
* There should be a propulsion system, that takes electricity and fuel (liquid, gaseous or solid) and turn it into heat, exhaust gas and acceleration
* There should be an FTL drive, that takes either electricity or fuel (liquid, gaseous or solid) and turn it into heat and bend space in front of the ship for higher accelerations and velocities
* There should be a control console that takes electricity and are needed to effectively control the propulsion and FTL drive
* The ship should loose heat via the surrounding space
* The ship should be controlled by the player either directly or indirectly by commands or auto-pilot
* The auto pilot takes electricity and can control propulsion and FTL
* Life support on board the ship takes electricity and converts carbon dioxed into oxygen and generates heat
* life support can either vent heat into space to cool the ship or generate additional heat so the ship doesn*t freeze
* A player consumes oxygen and produces carbon dioxide

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

