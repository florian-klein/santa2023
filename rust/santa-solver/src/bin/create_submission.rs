use log::info;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;

fn main() {
    env_logger::init();
    let solutions_path: String = "./../../data/solutions/".to_string();
    let submission_file_path: String = "./../../data/baseline.csv".to_string();
    let new_submission_file_path: String = "./../../data/submission_new.csv".to_string();
    let other_submissions_dir: String = "./../../data/improvement_complete_csvs".to_string();

    let mut overall_decrease = 0;

    // open submission file as csv
    let mut id_to_sol_string: HashMap<usize, String> =
        get_current_id_to_sol_string(&submission_file_path);
    for id in 0..398 {
        let csv_sol_path = format!("{}{}.csv", solutions_path, id);
        if !Path::new(&csv_sol_path).exists() {
            continue;
        }
        let min_path = get_min_path_for_id(&csv_sol_path).unwrap();
        let min_path_len = get_path_len(&min_path);

        let cur_min_path = id_to_sol_string.get(&id);
        let cur_min_path_len = get_path_len(&cur_min_path.unwrap());

        if min_path_len < cur_min_path_len {
            info!(
                "Found shorter path for id: {}, decrease of -{} from {} to {}",
                id,
                cur_min_path_len - min_path_len,
                cur_min_path_len,
                min_path_len
            );
            id_to_sol_string.insert(id, min_path);
            overall_decrease += cur_min_path_len - min_path_len;
        }
    }
    // find all other submission files in directory
    let mut other_submissions: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(other_submissions_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap().to_string();
        if path_str.ends_with(".csv") {
            other_submissions.push(path_str);
        }
    }
    for other_submission in other_submissions {
        let other_id_to_sol_string = read_other_id_to_sol_string_for_improvement(
            &other_submission,
            id_to_sol_string.clone(),
        );
        let other_score = score_id_to_sol_string(&other_id_to_sol_string);
        let current_score = score_id_to_sol_string(&id_to_sol_string);
        if other_score < current_score {
            info!(
                "Solutions in submission file: {} improved current solutions by {}",
                other_submission,
                current_score - other_score
            );
            overall_decrease += current_score - other_score;
            id_to_sol_string = other_id_to_sol_string;
        }
    }
    id_to_sol_string_to_csv(&new_submission_file_path, &id_to_sol_string);
    info!("-------------------");
    info!(
        "Overall decrease of {} steps in submission file",
        overall_decrease
    );
    info!(
        "The overall score will be {} for the entire submission file",
        score_id_to_sol_string(&id_to_sol_string)
    );
    info!("Find the submission file at: {}", new_submission_file_path);
    info!("-------------------");
}

pub fn get_path_len(path: &String) -> usize {
    return path.split(".").count();
}

pub fn get_min_path_for_id(path: &String) -> Option<String> {
    let mut reader = csv::Reader::from_path(path).unwrap();
    let mut min_path: Option<String> = None;
    for record in reader.records() {
        let record = record.unwrap();
        let record_path: String = record[1].to_string();
        let record_length: usize = record[2].parse::<usize>().unwrap();
        if let Some(ref path) = min_path {
            if record_length < path.len() {
                min_path = Some(record_path);
            }
        } else {
            min_path = Some(record_path);
        }
    }
    return min_path;
}

pub fn get_current_id_to_sol_string(path: &String) -> HashMap<usize, String> {
    let mut result: HashMap<usize, String> = HashMap::new();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for record in reader.records() {
        let record = record.unwrap();
        let id = record[0].parse::<usize>().unwrap();
        let sol = record[1].to_string();
        result.insert(id, sol);
    }
    return result;
}

pub fn read_other_id_to_sol_string_for_improvement(
    path: &String,
    current_map: HashMap<usize, String>,
) -> HashMap<usize, String> {
    let mut result: HashMap<usize, String> = HashMap::new();
    let mut reader = csv::Reader::from_path(path).unwrap();
    for record in reader.records() {
        let record = record.unwrap();
        let id = record[0].parse::<usize>().unwrap();
        let sol = record[1].to_string();
        if current_map.contains_key(&id) {
            let current_sol = current_map.get(&id).unwrap();
            if get_path_len(&sol) < get_path_len(&current_sol) {
                result.insert(id, sol);
            } else {
                result.insert(id, current_sol.to_string());
            }
        }
    }
    return result;
}

pub fn id_to_sol_string_to_csv(path: &String, id_to_sol_string: &HashMap<usize, String>) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut writer = csv::Writer::from_writer(file);
    writer.write_record(&["id", "moves"]).unwrap();
    for i in 0..398 {
        if id_to_sol_string.contains_key(&i) {
            let sol_string: String = id_to_sol_string.get(&i).unwrap().to_string();
            writer.write_record(&[i.to_string(), sol_string]).unwrap();
        }
    }
    writer.flush().unwrap();
}

pub fn score_id_to_sol_string(id_to_sol_string: &HashMap<usize, String>) -> usize {
    let mut score = 0;
    for i in 0..398 {
        if id_to_sol_string.contains_key(&i) {
            let sol_string: String = id_to_sol_string.get(&i).unwrap().to_string();
            let path_len = get_path_len(&sol_string);
            // info!("Puzzle with id {} has solution: {}", i, path_len);
            score += path_len;
        }
    }
    return score;
}
