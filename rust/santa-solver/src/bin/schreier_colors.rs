use log::{debug, error, info};
use santa_solver_lib::permutation::{self, Permutation};
use santa_solver_lib::puzzle::{self, Move, PuzzleType};
use santa_solver_lib::{minkwitz, schreier};
use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let solution_path = "./../../data/solutions/";
    let puzzle_info_path = if args.len() > 2 {
        &args[2]
    } else {
        "./../../data/puzzle_info.csv"
    };
    let puzzles_path = if args.len() > 3 {
        &args[3]
    } else {
        "./../../data/puzzles.csv"
    };

    // Load the puzzles
    debug!("Loading puzzle data...");
    let puzzles_info = puzzle::load_puzzle_info(puzzle_info_path).unwrap();
    let puzzles = puzzle::load_puzzles(puzzles_path, &puzzles_info).unwrap();
    // filter irrelevant puzzles
    let mut relevant_types: HashSet<PuzzleType> = HashSet::new();
    relevant_types.insert(PuzzleType::CUBE(4));
    for puzzle in puzzles {
        if !relevant_types.contains(&puzzle.puzzle_type) {
            continue;
        }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type,
        );

        // 1) Get the generators for the puzzle
        let puzzle_info_types = puzzles_info.get(&puzzle.puzzle_type).unwrap();
        let mut initial_gens: HashSet<minkwitz::PermAndWord> = HashSet::new();
        let mut index_to_gen_name = vec![];
        let mut index_to_perm: Vec<crate::permutation::Permutation> = Vec::new();
        let mut index: usize = 0;
        for move_elm in puzzle_info_types.iter() {
            let gen_perm_and_word = minkwitz::PermAndWord {
                perm: move_elm.permutation.clone(),
                word: vec![index],
                news: false,
                inverse: vec![],
            };
            initial_gens.insert(gen_perm_and_word);
            index_to_gen_name.push(move_elm.name.to_string());
            index_to_perm.push(move_elm.permutation.clone());
            index += 1;
        }

        debug!("Calculating color indices that need to be stabilized...");
        debug!("Goal String: {:?}", puzzle.goal_string);
        let color_indices_to_stabilize: Vec<HashSet<usize>> =
            schreier::SchreierSims::get_stabilizing_color_gens(&puzzle.goal_string);
        debug!("We need to stabilize {:?}", color_indices_to_stabilize);
        debug!("Calculating relaxed schreier sims for this problem...");
        let generators_for_target_group =
            schreier::SchreierSims::relaxed_schreier_sims(initial_gens, color_indices_to_stabilize);
        for gen in &generators_for_target_group {
            debug!("Generator: {:?}", gen);
            break;
        }

        // 2) Factorize the target permutation
        // let factorization =
        //     minkwitz::MinkwitzTable::factorize_minkwitz(&gens, &base, &sgs_table, &target);
        // if factorization.len() == 0 {
        //     continue;
        // }
        // let factorization_length = &factorization.len();
        // info!("----------------------------------------");
        // info!(
        //     "Found target path for this problem! Length: {:?}. Index Path is verified!",
        //     factorization_length
        // );
        // info!("Converting to String and writing to result paths...");
        // info!("----------------------------------------");
        // TestingUtils::assert_index_path_equals_permutation(&factorization, &target, &index_to_perm);
        // let path = PermutationPath::new(factorization);
        // let sol_string_dot_format = path.to_string(&index_to_gen_name);
        // let sol_path = format!("{}/{}.csv", solution_path, puzzle.id);
        // if !Path::new(&sol_path).exists() {
        //     let res = std::fs::File::create(&sol_path);
        //     if res.is_err() {
        //         error!("The file could not be written. Is your solution directory valid?");
        //     }
        //     let mut writer = csv::Writer::from_path(&sol_path).unwrap();
        //     writer.write_record(&["id", "moves", "length"]).unwrap();
        // }
        // let file = OpenOptions::new()
        //     .write(true)
        //     .create(true)
        //     .append(true)
        //     .open(&sol_path)
        //     .unwrap();
        // let mut writer = csv::Writer::from_writer(file);
        // writer
        //     .write_record(&[
        //         &puzzle.id.to_string(),
        //         &sol_string_dot_format,
        //         &factorization_length.to_string(),
        //     ])
        //     .unwrap();
        // debug!("Wrote file for this problem. Wrapping up... ")
    }
}
