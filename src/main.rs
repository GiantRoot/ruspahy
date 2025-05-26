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
    let config = config::load_config("assets/config.toml");
    let mut psys = ParticleSystem::new(&config);

    for step in 0..config.num_steps {
        psys.build_neighbor_list();
        psys.compute_forces();
        integrate(&mut psys, config.time_step);

        if step % config.output_interval == 0 {
            let filename = format!("output/step_{:05}.vtk", step);
            write_vtk(&psys, &filename);
            println!("Output: {}", filename);
        }
    }

    println!("Simulation completed.");
}