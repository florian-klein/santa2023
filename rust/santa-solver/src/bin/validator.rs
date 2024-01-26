use std::{fs::OpenOptions, path::Path};

use log::{debug, error, info};
use santa_solver_lib::{puzzle, testing_utils::TestingUtils};

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let puzzle_info_path = if args.len() > 1 {
        &args[1]
    } else {
        "./../../data/puzzle_info.csv"
    };
    let puzzles_path = if args.len() > 2 {
        &args[2]
    } else {
        "./../../data/puzzles.csv"
    };

    let solution_path = "./../../data/solutions/";

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_info).unwrap();
    let puzzle_types_to_solve = vec![puzzle::PuzzleType::CUBE(19)];

    for puzzle in puzzles {
        match puzzle.puzzle_type {
            puzzle::PuzzleType::CUBE(_) => {
                if !puzzle_types_to_solve.contains(&puzzle.puzzle_type) {
                    continue;
                }
            }
            _ => continue,
        }
        let init_string = puzzle.init_string;
        let goal_string = puzzle.goal_string;
        let wildcards = puzzle.num_wildcards;
        // if init_string.contains("N") {
        //     continue;
        // }
        // if puzzle.id != 30 {
        //     continue;
        // }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type,
        );
        // call the solver at ../call_cpp_solver.sh
        let output = std::process::Command::new("bash")
            .arg("./src/call_cpp_solver.sh")
            .arg(puzzle.id.to_string())
            .output()
            .expect("failed to execute process");
        let output_string = String::from_utf8(output.stdout).unwrap();
        let output_lines: Vec<&str> = output_string.split("\n").collect();
        let output_lines: Vec<&str> = output_lines
            .iter()
            .filter(|line| line.len() > 0)
            .map(|line| line.trim())
            .collect();
        let output_lines: Vec<&str> = output_lines;
        let solution = output_lines[0];
        // println!("Solution: {}", solution);
        // get solution length by splitting string on "." and counting length of resulting vector
        let solution_length = solution.split(".").collect::<Vec<&str>>().len();
        info!("Found solution of length {}", solution_length);
        // dont use join because it adds a "." at the end, better:
        // using iterator:
        TestingUtils::assert_applying_sol_string_to_initial_string_results_in_target(
            init_string,
            goal_string,
            solution.to_string(),
            puzzle.puzzle_type,
            wildcards,
        );
        let sol_path = format!("{}/{}.csv", solution_path, puzzle.id);
        if !Path::new(&sol_path).exists() {
            let res = std::fs::File::create(&sol_path);
            if res.is_err() {
                error!("The file could not be written. Is your solution directory valid?");
            }
            let mut writer = csv::Writer::from_path(&sol_path).unwrap();
            writer.write_record(&["id", "moves", "length"]).unwrap();
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&sol_path)
            .unwrap();
        let mut writer = csv::Writer::from_writer(file);
        writer
            .write_record(&[
                &puzzle.id.to_string(),
                &solution.to_string(),
                &solution_length.to_string(),
            ])
            .unwrap();
        debug!("Wrote file for this problem. Wrapping up... ")
    }
}
