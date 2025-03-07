use pyo3::prelude::*;
use std::collections::HashMap;
use std::thread;

fn calc_single_wire_img(node_net_info: HashMap<String, (usize, usize, usize, usize, usize, usize, f32)>, grid: usize) -> Vec<Vec<f32>> {
    let mut wire_img = vec![vec![0.; grid]; grid];
    for (start_x, start_y, end_x, end_y, base_offset_x, base_offset_y, weight) in node_net_info.values() {
        for i in 0..*start_x {
            let delta_hpwl = (start_x - i) as f32 * weight;
            for j in 0..grid {
                wire_img[i][j] = delta_hpwl;
            }
        }
        for j in 0..*start_y {
            let delta_hpwl = (start_y - j) as f32 * weight;
            for i in 0..grid {
                wire_img[i][j] = delta_hpwl;
            }
        }

        for i in *end_x..grid {
            let delta_hpwl = (i - end_x + *base_offset_x) as f32 * weight;
            for j in 0..grid {
                wire_img[i][j] = delta_hpwl;
            }
        }

        for j in *end_y..grid {
            let delta_hpwl = (j - end_y + *base_offset_y) as f32 * weight;
            for i in 0..grid {
                wire_img[i][j] = delta_hpwl;
            }
        }
    }
    wire_img
}


#[pyfunction]
fn calc_wire_img(node_net_info: HashMap<usize, HashMap<String, (usize, usize, usize, usize, usize, usize, f32)>>, grid: usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    let mut wire_img :Vec<Vec<Vec<f32>>> = Vec::with_capacity(8);
    for orient in 0..8 {
        let img = calc_single_wire_img(node_net_info.get(&orient).unwrap().clone(), grid);
        wire_img.push(img);
    }
    
    Ok(wire_img)
}

#[pyfunction]
fn calc_wire_img_parallel(node_net_info: HashMap<usize, HashMap<String, (usize, usize, usize, usize, usize, usize, f32)>>, grid: usize) -> PyResult<Vec<Vec<Vec<f32>>>> {
    
    let mut handles = Vec::with_capacity(8);
    for orient in 0..8 {
        let net_info = node_net_info.get(&orient).unwrap().clone();
        let new_grid = grid.clone();
        let handle = thread::spawn(move || {
            calc_single_wire_img(net_info, new_grid)
        });
        handles.push(handle);
    }
    
    let mut wire_img :Vec<Vec<Vec<f32>>> = Vec::with_capacity(8);
    for handle in handles {
        let img = handle.join().unwrap();
        wire_img.push(img);
    }
    Ok(wire_img)
}

/// A Python module implemented in Rust.
#[pymodule]
fn orientplace_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_wire_img, m)?)?;
    m.add_function(wrap_pyfunction!(calc_wire_img_parallel, m)?)?;
    Ok(())
}
