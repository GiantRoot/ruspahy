//! SPH 示例程序的入口。
//!
//! 主流程读取配置文件，初始化粒子系统，并在每个时间步
//! 按照光滑粒子法(SPH)的核函数计算相互作用，持续推进模
//! 拟并定期输出 VTK 文件以便可视化。

mod particle;
mod sph_kernel;
mod neighbor;
mod force;
mod integrator;
mod output;
mod config;
mod material;

use crate::particle::ParticleSystem;
use crate::output::write_vtk;
use crate::integrator::integrate;

fn main() {
    // 从配置文件读取模拟参数
    let config = config::load_config("assets/config.toml");
    // 创建初始的粒子集
    let mut psys = ParticleSystem::new(&config);

    // 主模拟循环
    for step in 0..config.num_steps {
        // 更新邻域列表并计算粒子间的力
        psys.build_neighbor_list();
        psys.compute_forces();
        // 对粒子位置和速度进行积分
        integrate(&mut psys, config.time_step);

        // 定期将粒子状态写入 VTK 文件，以便
        // 使用 Paraview 等工具视化
        if step % config.output_interval == 0 {
            let filename = format!("output/step_{:05}.vtk", step);
            write_vtk(&psys, &filename);
            println!("Output: {}", filename);
        }
    }

    println!("Simulation completed.");
}
