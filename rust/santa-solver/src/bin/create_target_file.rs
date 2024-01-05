use log::{debug, info};
use santa_solver_lib::permutation::Permutation;
use santa_solver_lib::puzzle;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    env_logger::init();
    /*
     * Valid Solution File Format:
     * id, moves
     * 0, f1.f2. ...
     * ...
     */
    /*
     * Target File Format:
     * id, target, target_length
     * 0, [2,0,4,1,3], 5
     * ...
     */
    let valid_solution_file_path = "./../../data/submission_schnack.csv".to_string();
    let target_file_path = "./../../data/target.csv".to_string();
    let puzzles_path = "./../../data/puzzles.csv".to_string();
    let puzzle_info_path = "./../../data/puzzle_info.csv".to_string();

    let target_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(Path::new(&target_file_path))
        .unwrap();

    let mut target_writer = csv::Writer::from_writer(target_file);
    target_writer
        .write_record(&["id", "target", "target_length"])
        .unwrap();

    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(&puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(&puzzles_path, &puzzles_info).unwrap();
    let id_to_move_string = get_id_to_move_string(valid_solution_file_path);
    for puzzle in puzzles {
        let mut gen_name_to_perm: HashMap<String, Permutation> = HashMap::new();
        for move_elm in puzzles_info.get(&puzzle.puzzle_type).unwrap().iter() {
            gen_name_to_perm.insert(move_elm.name.clone(), move_elm.permutation.clone());
        }
        let valid_move_string = id_to_move_string.get(&puzzle.id).unwrap();
        info!(
            "Calculating solution permutation for puzzle {}...",
            puzzle.id
        );
        let target_perm =
            get_permutation_from_operation_string(&valid_move_string, &gen_name_to_perm);
        let target = target_perm.p;
        let target_length = target.len();
        target_writer
            .write_record(&[
                &puzzle.id.to_string(),
                &target
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
                &target_length.to_string(),
            ])
            .unwrap();
        info!("Writing solution permutation for puzzle {} done", puzzle.id);
    }
    target_writer.flush().unwrap();
    info!("Writing target file done");
}

pub fn get_id_to_move_string(solution_path: String) -> HashMap<usize, String> {
    let mut id_to_move_string: HashMap<usize, String> = HashMap::new();
    let solution_file = OpenOptions::new()
        .read(true)
        .open(Path::new(&solution_path))
        .unwrap();
    let mut reader = csv::Reader::from_reader(solution_file);
    for result in reader.records() {
        let record = result.unwrap();
        debug!("Record: {:?}", record);
        let id: usize = record[0].parse().unwrap();
        let move_string = record[1].to_string();
        id_to_move_string.insert(id, move_string);
    }
    id_to_move_string
}

pub fn get_permutation_from_operation_string(
    op_str: &String,
    gen_name_to_perm: &HashMap<String, Permutation>,
) -> Permutation {
    let n = gen_name_to_perm.iter().next().unwrap().1.len();
    let mut result = Permutation::identity(n);
    let operations = op_str.split(".");
    for operation in operations {
        let next_perm = &gen_name_to_perm.get(operation).unwrap();
        result = result.compose(&next_perm);
    }
    result.inverse()
}
