//! 邻域搜索工具。
//!
//! 使用简单的均匀网格构建邻域列表，当前以粒子的平均间距
//! 作为搜索半径。该实现避免了全对遍历，在粒子数较多时
//! 能够显著提高效率。

use std::collections::HashMap;
use crate::particle::Particle;

/// 三维网格单元索引
type Cell = (i64, i64, i64);

/// 为所有粒子构建邻域列表。
///
/// `radius` 为搜索半径，通常与平滑长度一致。
pub fn build_neighbor_list(particles: &[Particle], radius: f64) -> Vec<Vec<usize>> {
    let cell_size = radius;
    let mut grid: HashMap<Cell, Vec<usize>> = HashMap::new();

    for (i, p) in particles.iter().enumerate() {
        grid.entry(cell_index(p.position, cell_size))
            .or_default()
            .push(i);
    }

    let mut neighbors = vec![Vec::new(); particles.len()];
    let r2 = radius * radius;

    for (i, p) in particles.iter().enumerate() {
        let base = cell_index(p.position, cell_size);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let cell = (
                        base.0 + dx as i64,
                        base.1 + dy as i64,
                        base.2 + dz as i64,
                    );
                    if let Some(list) = grid.get(&cell) {
                        for &j in list {
                            if i == j {
                                continue;
                            }
                            if squared_distance(p.position, particles[j].position) <= r2 {
                                neighbors[i].push(j);
                                if !neighbors[j].contains(&i) {
                                    neighbors[j].push(i);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    neighbors
}

fn cell_index(pos: [f64; 3], size: f64) -> Cell {
    (
        (pos[0] / size).floor() as i64,
        (pos[1] / size).floor() as i64,
        (pos[2] / size).floor() as i64,
    )
}

fn squared_distance(a: [f64; 3], b: [f64; 3]) -> f64 {
    (a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)
}
