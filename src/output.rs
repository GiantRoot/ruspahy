//! 输出工具。
//!
//! 目前仅提供一个非常简单的 VTK 写出器，用于可视化粒子集合。

use crate::particle::ParticleSystem;
use std::fs::File;
use std::io::{BufWriter, Write};

/// 将粒子位置和压力写入 VTK 文件。
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

    writeln!(writer, "\nSCALARS stress float 1").unwrap();
    writeln!(writer, "LOOKUP_TABLE default").unwrap();
    for p in &psys.particles {
        writeln!(writer, "{}", p.stress).unwrap();
    }

    writeln!(writer, "\nSCALARS material_id int 1").unwrap();
    writeln!(writer, "LOOKUP_TABLE default").unwrap();
    for p in &psys.particles {
        writeln!(writer, "{}", p.material_id).unwrap();
    }

    writeln!(writer, "\nSCALARS material_type int 1").unwrap();
    writeln!(writer, "LOOKUP_TABLE default").unwrap();
    for p in &psys.particles {
        let t = psys.materials[p.material_id].material_type as i32;
        writeln!(writer, "{}", t).unwrap();
    }
}
