use log::{debug, error, info};
use santa_solver_lib::minkwitz;
use santa_solver_lib::minkwitz::TransTable;
use santa_solver_lib::permutation;
use santa_solver_lib::puzzle;
use santa_solver_lib::puzzle::PuzzleType;
use std::fs;
use std::io::Read;
fn create_sgs_table_wrapper(
    puzzle: puzzle::Puzzle,
    gens: &minkwitz::GroupGens,
    base: &minkwitz::GroupBase,
    minkwitz_tables_path: String,
) -> TransTable {
    info!(
        "Creating new SGS table for puzzle_type {:?}",
        puzzle.puzzle_type,
    );
    let sgs_table = minkwitz::MinkwitzTable::build_short_word_sgs(&gens, &base, 1000, 100, 100);
    return sgs_table;
}

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let use_existing_tables = if args.len() > 1 { &args[1] } else { "false" };
    let minkwitz_tables_path = "./../../data/minkwitz_tables";
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
        let mut should_break = false;
        for i in 0..35 {
            if puzzle.puzzle_type == PuzzleType::CUBE(i) {
                should_break = true;
            }
        }
        if should_break {
            continue;
        }
        info!(
            "Solving puzzle {} of type {:?}",
            puzzle.id, puzzle.puzzle_type,
        );
        let target = permutation::get_permutation(&puzzle.initial_state, &puzzle.goal_state);
        let target_info = target.compute_info();
        debug!("We want to reach following target: {:?}", target_info);

        // 1) Generate Strong Generating Set Table for the group
        let puzzle_info_types = puzzles_info.get(&puzzle.puzzle_type).unwrap();
        let mut gens = minkwitz::GroupGens::new(vec![]);
        for move_elm in puzzle_info_types.iter() {
            let new_gen =
                minkwitz::GroupGen::new(move_elm.name.clone(), move_elm.permutation.clone());
            gens.add(new_gen);
        }
        let base_vec = (0..target.len()).collect::<Vec<_>>();
        // todo: find a better base using schreier-sims algorithm
        let base = minkwitz::GroupBase::new(base_vec);
        let sgs_table: TransTable =
            create_sgs_table_wrapper(puzzle, &gens, &base, minkwitz_tables_path.to_string());

        // 2) Factorize the target permutation
        let factorization =
            minkwitz::MinkwitzTable::factorize_minkwitz(&gens, &base, &sgs_table, &target);
        info!("----------------------------------------");
        info!(
            "Found target path for this problem! Length: {:?} (todo: verify!!)",
            factorization.len()
        );
        info!("----------------------------------------");
    }
}
