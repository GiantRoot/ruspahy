//! Entry point for the SPH demo.
//!
//! The main routine loads the simulation configuration, initializes the
//! particle system and iteratively advances the simulation while writing
//! VTK files for visualization.

mod particle;
mod sph_kernel;
mod neighbor;
mod force;
mod integrator;
mod output;
mod config;

use crate::particle::ParticleSystem;
use crate::output::write_vtk;
use crate::integrator::integrate;

fn main() {
    // Read parameters from the configuration file.
    let config = config::load_config("assets/config.toml");
    // Create the initial particle set.
    let mut psys = ParticleSystem::new(&config);

    // Main simulation loop.
    for step in 0..config.num_steps {
        // Update neighbor list and compute forces for each particle.
        psys.build_neighbor_list();
        psys.compute_forces();
        // Integrate particle positions and velocities.
        integrate(&mut psys, config.time_step);

        // Periodically write the particle state to a VTK file so it can
        // be visualized with tools such as Paraview.
        if step % config.output_interval == 0 {
            let filename = format!("output/step_{:05}.vtk", step);
            write_vtk(&psys, &filename);
            println!("Output: {}", filename);
        }
    }

    println!("Simulation completed.");
}
