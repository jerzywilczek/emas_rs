pub fn default_combat(this_agent_fitness: f64, other_agent_fitness: f64) -> f64 {
    if this_agent_fitness < other_agent_fitness {
        return 0.8;
    }
    0.2
}
