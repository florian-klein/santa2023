use log::info;

pub fn get_moves_to_solve(puzzle: &crate::puzzle::Puzzle) -> Vec<usize> {
    match puzzle.puzzle_type {
        crate::puzzle::PuzzleType::CUBE(n) => {
            info!("Using Cube Base!");
            return crate::coordinate_calc::cube::get_cube_order_to_traverse(n);
        }
        crate::puzzle::PuzzleType::GLOBE(m, n) => {
            return crate::coordinate_calc::globe::get_globe_order_to_traverse(m, n);
        }
        crate::puzzle::PuzzleType::WREATH(_n) => {
            todo!("Coordinate mapping for this type is not implemented yet!");
        }
    }
}
