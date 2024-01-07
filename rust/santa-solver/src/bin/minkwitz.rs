use log::{debug, error, info};
use santa_solver_lib::minkwitz::{PermAndWord, TransTable};
use santa_solver_lib::permutation::PermutationPath;
use santa_solver_lib::permutation::{self, Permutation};
use santa_solver_lib::puzzle::{self, Move, PuzzleType};
use santa_solver_lib::schreier::SchreierSims;
use santa_solver_lib::{minkwitz, minkwitz_search, schreier};
use std::collections::{HashMap, HashSet};
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
    let sgs_table = minkwitz::MinkwitzTable::build_short_word_sgs(&gens, &base, 1000, 100, 50);
    return sgs_table;
}

#[allow(dead_code)]
fn get_base_check_if_exists(
    puzzle: &puzzle::Puzzle,
    puzzle_info: &HashMap<PuzzleType, Vec<Move>>,
    bases_path: &str,
) -> Option<minkwitz::GroupBase> {
    let base_path = format!("{}/{}.csv", bases_path, puzzle.puzzle_type);
    let perm_size = puzzle.initial_state.len();
    if Path::new(&base_path).exists() {
        let base = minkwitz::GroupBase::load_from_file(&base_path);
        info!(
            "We found an existing base of length {:?} vs perm_size {:?} for this puzzle of type {:?}. Loading it...",
            base.elements.len(),
            perm_size,
            puzzle.puzzle_type,
        );
        return Some(base);
    }
    info!(
        "We did not find an existing base for this puzzle of type {:?}. Creating it...",
        puzzle.puzzle_type
    );
    let mut index_to_perm: Vec<crate::permutation::Permutation> = Vec::new();
    for move_elm in puzzle_info.get(&puzzle.puzzle_type).unwrap() {
        index_to_perm.push(move_elm.permutation.clone());
    }
    let base_vec = SchreierSims::find_base(index_to_perm);
    let base = minkwitz::GroupBase::new(base_vec);
    base.write_to_file(&base_path);
    info!(
        "Base of length {:?} vs. perm_size {:?} created and written to file.",
        base.elements.len(),
        perm_size
    );
    return Some(base);
}

fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let solution_path = "./../../data/solutions/";
    let _bases_storage_path = "./../../data/bases/";
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
    debug!("Loading id to target hashmap...");
    let id_to_target: HashMap<usize, Permutation> =
        puzzle::load_id_to_target_permutation("./../../data/target.csv").unwrap();
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
        let target_perm = id_to_target
            .get(&puzzle.id)
            .expect("Could not find target for this puzzle id!");
        debug!("Target permutation: {:?}", target_perm);
        let target = target_perm.inverse();
        let target_info = target.compute_info();
        debug!("We want to reach following target: {:?}", target_info);

        // 1) Generate Strong Generating Set Table for the group
        let puzzle_info_types = puzzles_info.get(&puzzle.puzzle_type).unwrap();
        let mut gens = minkwitz::GroupGens::new(vec![]);
        let mut index_to_gen_name = vec![];
        let mut index_to_perm: Vec<crate::permutation::Permutation> = Vec::new();
        let mut str_to_gen: HashMap<String, Permutation> = HashMap::new();
        for move_elm in puzzle_info_types.iter() {
            let new_gen =
                minkwitz::GroupGen::new(move_elm.name.clone(), move_elm.permutation.clone());
            gens.add(new_gen);
            index_to_gen_name.push(move_elm.name.to_string());
            index_to_perm.push(move_elm.permutation.clone());
            str_to_gen.insert(move_elm.name.clone(), move_elm.permutation.clone());
        }
        // let base = get_base_check_if_exists(&puzzle, &puzzles_info, bases_storage_path).unwrap();
        let base_vec: Vec<usize> = (0..puzzle.initial_state.len()).collect();
        let base = minkwitz::GroupBase::new(base_vec);
        let sgs_table: TransTable = create_sgs_table_wrapper(&puzzle, &gens, &base);

        // 2) Factorize the target permutation
        let valid_indices: Vec<HashSet<usize>> =
            schreier::SchreierSims::get_stabilizing_color_gens(&puzzle.goal_string);
        let target_pw = PermAndWord::new(target.clone(), vec![]);
        info!("Searching for a path to the target permutation...");
        let fact = minkwitz_search::minkwitz_djikstra(valid_indices.clone(), target_pw, sgs_table);
        if fact.is_none() {
            error!("Could not find a path to the target permutation!");
            continue;
        }
        let mut factorization = fact.unwrap().word;
        factorization.reverse();

        if factorization.len() == 0 {
            return;
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
}
