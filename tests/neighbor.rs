use ruspahy::particle::ParticleSystem;
use ruspahy::config::SimConfig;

#[test]
fn neighbor_single_particle() {
    let config = SimConfig {
        grid: [1, 1, 1],
        spacing: 0.1,
        time_step: 0.01,
        num_steps: 1,
        output_interval: 1,
        materials: Vec::new(),
        interfaces: Vec::new(),
        spheres: Vec::new(),
    };

    let mut ps = ParticleSystem::new(&config);
    ps.build_neighbor_list();
    assert_eq!(ps.neighbors.len(), 1);
    assert!(ps.neighbors[0].is_empty());
}

#[test]
fn neighbor_small_spacing() {
    let config = SimConfig {
        grid: [2, 1, 1],
        spacing: 1e-7,
        time_step: 0.01,
        num_steps: 1,
        output_interval: 1,
        materials: Vec::new(),
        interfaces: Vec::new(),
        spheres: Vec::new(),
    };

    let mut ps = ParticleSystem::new(&config);
    ps.build_neighbor_list();
    assert_eq!(ps.neighbors.len(), 2);
    assert_eq!(ps.neighbors[0], vec![1]);
    assert_eq!(ps.neighbors[1], vec![0]);
}
