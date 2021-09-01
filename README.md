# Petri-world
A Neural-Network + Genetic Algorithm powered simulation powered by Rust

## Goals

- [x] Have working NN and GA crates
- [x] Refactor away from `rand` crate to use `fastrand` instead. Encapsulate RNG code into its own crate.
- [ ] Use Bevy engine as driver for MVP simulation
- [ ] Pause/Skip/FastForward simulation
- [ ] Add ability to customise parameters of simulation
  - [ ] Adjust GA params
  - [ ] Adjust NN params
    - [ ] Flexible, user-defined NN topologies
    - [ ] More kinds of Neuron types (currently only PReLU supported)
  - [ ] Multiple kinds of populations
    - [ ] Customise interactions/fitness scoring of each population
