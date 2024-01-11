use std::{collections::HashMap, usize};

use crate::coordinate_calc::cube;

fn create_cube_pos(
    vector: Vec<usize>,
    n: usize,
) -> Result<HashMap<char, Vec<usize>>, &'static str> {
    if vector.len() != 6 * n * n {
        return Err(
            "The length of the vector is not compatible with the specified 'n' for an nxnxn cube.",
        );
    }

    let faces = ['A', 'B', 'C', 'D', 'E', 'F'];
    let face_vectors: Vec<_> = faces
        .iter()
        .enumerate()
        .map(|(i, _)| vector[i * n * n..(i + 1) * n * n].to_vec())
        .collect();
    let cube_dict: HashMap<_, _> = faces.iter().cloned().zip(face_vectors).collect();

    Ok(cube_dict)
}

fn find_center_elements(n: usize) -> Vec<usize> {
    if n % 2 == 1 {
        vec![(n * n) / 2]
    } else {
        let mid = n / 2;
        vec![
            mid * n + mid - 1,
            mid * n + mid,
            (mid - 1) * n + mid,
            (mid - 1) * n + mid - 1,
        ]
    }
}

fn create_centers_pos(
    cube_dict: &HashMap<char, Vec<usize>>,
    n: usize,
) -> HashMap<char, Vec<usize>> {
    let center_indices = find_center_elements(n);
    let centers: HashMap<_, _> = cube_dict
        .iter()
        .map(|(&face, values)| (face, center_indices.iter().map(|&i| values[i]).collect()))
        .collect();
    centers
}

fn create_corners_pos(
    cube_dict: &HashMap<char, Vec<usize>>,
    n: usize,
) -> HashMap<char, HashMap<&str, usize>> {
    let corners: HashMap<_, _> = cube_dict
        .iter()
        .map(|(&face, values)| {
            (
                face,
                [
                    ("top-left", values[0]),
                    ("top-right", values[n - 1]),
                    ("bottom-left", values[n * (n - 1)]),
                    ("bottom-right", values[n * n - 1]),
                ]
                .iter()
                .cloned()
                .map(|(key, value)| (key, value))
                .collect(),
            )
        })
        .collect();
    corners
}

fn create_edges_pos(cube_dict: &HashMap<char, Vec<usize>>, n: usize) -> HashMap<char, Vec<usize>> {
    if n < 3 {
        let edges: HashMap<_, _> = cube_dict
            .iter()
            .map(|(&face, values)| {
                (
                    face,
                    [
                        ("top", (values[0], values[1])),
                        ("right", (values[1], values[3])),
                        ("bottom", (values[2], values[3])),
                        ("left", (values[0], values[2])),
                    ]
                    .iter()
                    .cloned()
                    .flat_map(|(_, (v1, v2))| vec![v1, v2])
                    .collect(),
                )
            })
            .collect();
        edges
    } else {
        let edges: HashMap<_, _> = cube_dict
            .iter()
            .map(|(&face, values)| {
                let mut face_edges = Vec::new();
                face_edges.extend_from_slice(&values[1..n - 1]); // Top edge
                face_edges.extend((1..n - 1).map(|i| values[i * n + n - 1])); // Right edge
                face_edges.extend((1..n - 1).map(|i| values[n * (n - 1) + i])); // Bottom edge
                face_edges.extend((1..n - 1).map(|i| values[i * n])); // Left edge
                (face, face_edges)
            })
            .collect();
        edges
    }
}

// pub fn get_center_indices(cube_size: usize) -> Vec<usize> {
//     // af ed bc
//     let solution_states = (1..=6 * cube_size * cube_size).collect::<Vec<_>>();
//     let cube_side_pos = create_cube_pos(solution_states, cube_size).unwrap();
//     let corner_pos: HashMap<char, HashMap<&str, usize>> =
//         create_corners_pos(&cube_side_pos, cube_size);
//     for letter in ['A', 'F', 'E', 'D', 'B', 'C'].iter() {
//         let center = corner_pos[letter]["bottom-right"];
//         println!("{}", center);
//     }
//
//     return;
// }

pub fn get_layers(cube_size: usize) -> Vec<Vec<usize>> {
    let mut layers: Vec<Vec<usize>> = Vec::new();
    let letters: Vec<char> = vec!['E', 'B', 'C', 'D'];
    let solution_state: Vec<usize> = (1..=6 * cube_size * cube_size).collect();
    let cube_side_pos = create_cube_pos(solution_state.clone(), cube_size).unwrap();
    let corner_pos = create_corners_pos(&cube_side_pos, cube_size);
    for lay_num in 0..cube_size {
        let mut layer = Vec::new();
        for letter in letters.iter() {
            let bottom_left = corner_pos[letter]["bottom-left"] - lay_num * cube_size;
            let bottom_right = corner_pos[letter]["bottom-right"] - lay_num * cube_size;
            for i in (bottom_left - 1)..bottom_right {
                layer.push(i);
            }
        }
        layers.push(layer);
    }
    layers
}

pub fn get_cube_order_to_traverse(cube_size: usize) -> Vec<usize> {
    let layers = get_layers(cube_size);
    let mut result = Vec::new();
    // solve the center pieces first to reduce the cube to a 3x3x3 or 2x2x2
    let sol_state = (1..=6 * cube_size * cube_size).collect();
    let cube_side_pos: HashMap<char, Vec<usize>> = create_cube_pos(sol_state, cube_size).unwrap();
    for letter in ['A', 'F', 'E', 'D', 'B', 'C'] {
        let cur_cube_side = cube_side_pos.get(&letter);
        for (i, elm) in cur_cube_side.unwrap().iter().enumerate() {
            if i < cube_size || i >= (cube_size - 1) * cube_size {
                continue;
            }
            if i % cube_size == 0 || i % cube_size == cube_size - 1 {
                continue;
            }
            result.push(*elm);
        }
    }
    for (laynum, layer) in layers.iter().enumerate() {
        if laynum == 0 || laynum == cube_size - 1 {
            continue;
        }
        for i in 1..layer.len() - 1 {
            let center = layer[i];
            result.push(center);
        }
    }
    for (i, layer) in layers.iter().enumerate() {
        if i != 0 && i != cube_size - 1 {
            let left_corner = layer[0];
            let right_corner = layer[layer.len() - 1];
            result.push(left_corner);
            result.push(right_corner);
        } else {
            result.extend(layer);
        }
    }
    let f_res = result.iter().map(|x| x - 1).collect::<Vec<_>>();
    f_res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_creation() {
        let cube_size = 4;
        let solution_state: Vec<usize> = (1..=6 * cube_size * cube_size).collect();
        let cube_side_pos = create_cube_pos(solution_state.clone(), cube_size).unwrap();
        let centers_pos = create_centers_pos(&cube_side_pos, cube_size);
        let corners_pos = create_corners_pos(&cube_side_pos, cube_size);
        let edges_pos = create_edges_pos(&cube_side_pos, cube_size);

        // Assert or check the results here
        assert_eq!(cube_side_pos.len(), 6);
        assert_eq!(centers_pos.len(), 6);
        assert_eq!(corners_pos.len(), 6);
        assert_eq!(edges_pos.len(), 6);
    }

    #[test]
    fn test_get_layers() {
        let cube_size = 2;
        let layers = get_layers(cube_size);
        let expected_layers = vec![
            vec![18, 19, 6, 7, 10, 11, 14, 15],
            vec![16, 17, 4, 5, 8, 9, 12, 13],
        ];
        assert_eq!(layers, expected_layers);
    }

    #[test]
    fn test_get_cube_order_to_traverse() {
        let cube_size = 4;
        let cube_order = get_cube_order_to_traverse(cube_size);
        let expected_order = vec![
            5, 6, 9, 10, 85, 86, 89, 90, 69, 70, 73, 74, 53, 54, 57, 58, 21, 22, 25, 26, 37, 38,
            41, 42, 72, 73, 74, 23, 24, 25, 26, 39, 40, 41, 42, 55, 56, 57, 68, 69, 70, 19, 20, 21,
            22, 35, 36, 37, 38, 51, 52, 53, 75, 76, 77, 78, 27, 28, 29, 30, 43, 44, 45, 46, 59, 60,
            61, 62, 71, 58, 67, 54, 63, 64, 65, 66, 15, 16, 17, 18, 31, 32, 33, 34, 47, 48, 49, 50,
        ];
        assert_eq!(cube_order, expected_order);
    }
}
