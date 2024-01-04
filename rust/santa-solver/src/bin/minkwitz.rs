use log::{debug, error, info};
use santa_solver_lib::minkwitz;
use santa_solver_lib::minkwitz::TransTable;
use santa_solver_lib::permutation;
use santa_solver_lib::permutation::PermutationPath;
use santa_solver_lib::puzzle::{self, PuzzleType};
use std::fs::OpenOptions;
use std::path::Path;
fn create_sgs_table_wrapper(
    puzzle: &puzzle::Puzzle,
    gens: &minkwitz::GroupGens,
    base: &minkwitz::GroupBase,
) -> TransTable {
    info!(
        "Creating new SGS table for puzzle_type {:?}",
        puzzle.puzzle_type,
    );
    let sgs_table = minkwitz::MinkwitzTable::build_short_word_sgs(&gens, &base, 1500, 100, 100);
    return sgs_table;
}

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
    for puzzle in puzzles {
        if puzzle
            .initial_state
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != puzzle.initial_state.len()
        {
            continue;
        }
        let mut should_continue = false;
        for i in 0..34 {
            if puzzle.puzzle_type == PuzzleType::CUBE(i) {
                should_continue = true;
                break;
            }
        }
        // if puzzle.puzzle_type == PuzzleType::GLOBE(3, 33) {
        //     should_continue = true;
        // }
        if puzzle.puzzle_type == PuzzleType::GLOBE(33, 3) {
            should_continue = true;
        }
        if should_continue {
            continue;
        }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type,
        );
        let target =
            permutation::get_permutation(&puzzle.initial_state, &puzzle.goal_state).inverse();
        let target_info = target.compute_info();
        debug!("We want to reach following target: {:?}", target_info);

        // 1) Generate Strong Generating Set Table for the group
        let puzzle_info_types = puzzles_info.get(&puzzle.puzzle_type).unwrap();
        let mut gens = minkwitz::GroupGens::new(vec![]);
        let mut index_to_gen_name = vec![];
        let mut index_to_perm: Vec<crate::permutation::Permutation> = Vec::new();
        for move_elm in puzzle_info_types.iter() {
            let new_gen =
                minkwitz::GroupGen::new(move_elm.name.clone(), move_elm.permutation.clone());
            gens.add(new_gen);
            index_to_gen_name.push(move_elm.name.to_string());
            index_to_perm.push(move_elm.permutation.clone());
        }
        let base_vec = (0..target.len()).collect::<Vec<_>>();
        // todo: find a better base using schreier-sims algorithm
        let base = minkwitz::GroupBase::new(base_vec);
        let sgs_table: TransTable = create_sgs_table_wrapper(&puzzle, &gens, &base);

        // 2) Factorize the target permutation
        let factorization =
            minkwitz::MinkwitzTable::factorize_minkwitz(&gens, &base, &sgs_table, &target);
        if factorization.len() == 0 {
            continue;
        }
        let factorization_length = &factorization.len();
        info!("----------------------------------------");
        info!(
            "Found target path for this problem! Length: {:?}. Index Path is verified!",
            factorization_length
        );
        info!("Converting to String and writing to result paths...");
        info!("----------------------------------------");
        let path = PermutationPath::new(factorization);
        let sol_string_dot_format = path.to_string(&index_to_gen_name);
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
                &sol_string_dot_format,
                &factorization_length.to_string(),
            ])
            .unwrap();
        debug!("Wrote file for this problem. Wrapping up... ")
    }
    main();
}
