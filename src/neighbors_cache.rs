// Сгенерировано ИИ с любовью

use std::io::Write;
use crate::resources::*;
use crate::components::*;

use std::fs;
use std::path::Path;
use std::io;

pub fn load_or_compute_connections(
    conf: &Config,
    sorted_cells: &SortedCellsByIDS,
) -> Vec<Vec<usize>> {
    let cache_dir = "cache";
    fs::create_dir_all(cache_dir).unwrap();
    
    // Уникальное имя файла на основе параметров сетки
    let cache_path = format!(
        "{}/grid_{}_{}_obs_{}.bin", 
        cache_dir, conf.grid.x_amount, conf.grid.y_amount, conf.observ_size
    );

    if Path::new(&cache_path).exists() {
        println!("✅ Загрузка кэша соединений из {}...", cache_path);
        let bytes = fs::read(&cache_path).expect("Failed to read cache");
        bincode::deserialize(&bytes).expect("Failed to deserialize cache")
    } else {
        println!("⚙️  Генерация и кэширование соединений (это может занять время)...");
        let total = conf.grid.size() as usize;
        let mut cached_neighbors: Vec<Vec<usize>> = Vec::with_capacity(total);
        
        for (cid, _) in sorted_cells.0.iter().enumerate() {
            if cid % (total / 20).max(1) == 0 {
                let progress = (cid as f32 / total as f32 * 100.0) as usize;
                let bars = progress / 2;
                print!("\r[{}{}] {}%", "█".repeat(bars), " ".repeat(50 - bars), progress);
                io::stdout().flush().unwrap();
            }

            let seria = Seria::new(sorted_cells, &conf.grid, cid, conf.observ_size + 1);
            
            // Кэшируем ТОЛЬКО CellID! Entity добавим позже.
            let mut nbors_cell_ids = vec![cid]; 
            for obs_cell in seria {
                if obs_cell != cid {
                    nbors_cell_ids.push(obs_cell);
                }
            }
            cached_neighbors.push(nbors_cell_ids);
        }
        
        println!("\n💾 Сохранение кэша в {}...", cache_path);
        let bytes = bincode::serialize(&cached_neighbors).expect("Failed to serialize");
        fs::write(&cache_path, bytes).expect("Failed to write cache");
        println!("✅ Кэширование завершено!");
        
        cached_neighbors
    }
}