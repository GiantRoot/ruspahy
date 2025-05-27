use ruspahy::particle::{ParticleSystem, Particle};
use ruspahy::material::{Material, MaterialType};
use ruspahy::integrator::integrate;
use ruspahy::output::write_vtk;

fn add_sphere(psys: &mut ParticleSystem, center: [f64;3], radius: f64, spacing: f64, velocity: [f64;3], material_id: usize) {
    let n = (radius / spacing).ceil() as i32;
    let r2 = radius * radius;
    for ix in -n..=n {
        for iy in -n..=n {
            for iz in -n..=n {
                let dx = ix as f64 * spacing;
                let dy = iy as f64 * spacing;
                let dz = iz as f64 * spacing;
                if dx*dx + dy*dy + dz*dz <= r2 {
                    psys.particles.push(Particle {
                        position: [center[0]+dx, center[1]+dy, center[2]+dz],
                        velocity,
                        force: [0.0;3],
                        density: 1000.0,
                        pressure: 0.0,
                        stress: 0.0,
                        material_id,
                    });
                }
            }
        }
    }
}

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

    let materials = vec![
        Material {
            id: 0,
            name: "steel".to_string(),
            material_type: MaterialType::Elastoplastic,
            density: 7800.0,
            youngs_modulus: 2e11,
            yield_strength: Some(2e8),
            hardening_modulus: None,
            damage_threshold: None,
        },
        Material {
            id: 1,
            name: "copper".to_string(),
            material_type: MaterialType::Elastoplastic,
            density: 8900.0,
            youngs_modulus: 1.1e11,
            yield_strength: Some(1e8),
            hardening_modulus: None,
            damage_threshold: None,
        },
    ];

    let mut psys = ParticleSystem {
        particles: Vec::new(),
        neighbors: Vec::new(),
        materials: materials.clone(),
    };

    add_sphere(&mut psys, center1, radius, spacing, velocity1, 0);
    add_sphere(&mut psys, center2, radius, spacing, velocity2, 1);

    psys.neighbors = vec![Vec::new(); psys.particles.len()];

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
