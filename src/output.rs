//! Output utilities.
//!
//! Currently only a very simple VTK writer is provided for visualizing the
//! particle set.

use crate::particle::ParticleSystem;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Write the particle positions and pressure values to a VTK file.
pub fn write_vtk(psys: &ParticleSystem, filename: &str) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# vtk DataFile Version 3.0").unwrap();
    writeln!(writer, "SPH particles").unwrap();
    writeln!(writer, "ASCII").unwrap();
    writeln!(writer, "DATASET POLYDATA").unwrap();
    writeln!(writer, "POINTS {} float", psys.particles.len()).unwrap();

    for p in &psys.particles {
        writeln!(writer, "{} {} {}", p.position[0], p.position[1], p.position[2]).unwrap();
    }

    writeln!(writer, "\nPOINT_DATA {}", psys.particles.len()).unwrap();
    writeln!(writer, "SCALARS pressure float 1").unwrap();
    writeln!(writer, "LOOKUP_TABLE default").unwrap();
    for p in &psys.particles {
        writeln!(writer, "{}", p.pressure).unwrap();
    }

    writeln!(writer, "\nSCALARS material_id int 1").unwrap();
    writeln!(writer, "LOOKUP_TABLE default").unwrap();
    for p in &psys.particles {
        writeln!(writer, "{}", p.material_id).unwrap();
    }
}
