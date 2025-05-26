use ruspahy::particle::ParticleSystem;
use ruspahy::integrator::integrate;
use ruspahy::output::write_vtk;

fn main() {
    let spacing = 0.1;
    let time_step = 0.001;
    let num_steps = 100;
    let output_interval = 10;
    let radius = 0.5;

    let center1 = [-1.0, 0.0, 0.0];
    let center2 = [1.0, 0.0, 0.0];
    let velocity1 = [1.0, 0.0, 0.0];
    let velocity2 = [-1.0, 0.0, 0.0];

    let mut psys = ParticleSystem::two_spheres(
        center1,
        center2,
        radius,
        spacing,
        velocity1,
        velocity2,
        0,
        0,
    );

    for step in 0..num_steps {
        psys.build_neighbor_list();
        psys.compute_forces();
        integrate(&mut psys, time_step);

        if step % output_interval == 0 {
            let filename = format!("output/spheres_{:05}.vtk", step);
            write_vtk(&psys, &filename);
            println!("Output: {}", filename);
        }
    }

    println!("Simulation completed.");
}
