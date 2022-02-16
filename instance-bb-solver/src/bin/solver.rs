use std::{
    collections::HashSet,
    env,
    fs::{read_dir, DirEntry, File},
    io::{BufRead, BufReader, ErrorKind},
};

use instance_bb_solver::{ReadBenchmarkImpl, ReadingInfo};
use solving_manager::utils::perform_multiple_benchmarks;

fn already_solved_instances(log_file: &String) -> HashSet<String> {
    let file = File::open(log_file);

    match file {
        Ok(file) => {
            let file_lines = BufReader::new(file).lines();

            let instance_lines = file_lines.skip(1);

            instance_lines
                .map(|instance_line| {
                    instance_line
                        .expect(&format!("error while reading a line from {}", log_file))
                        .split_whitespace()
                        .next()
                        .expect(&format!("unexpected malformed line in {}", log_file))
                        .to_string()
                })
                .collect()
        }
        Err(err) if err.kind() == ErrorKind::NotFound => HashSet::new(),
        Err(err) => panic!("{}", err),
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let num_workers = args
        .next()
        .expect("at least three arguments should be provided")
        .parse()
        .expect("first argument should be the number of workers");

    let output_file = args
        .next()
        .expect("at least three arguments should be proided");

    let instances_directory = args
        .next()
        .expect("at least three arguments should be provided");

    let already_solved_instances = already_solved_instances(&output_file);

    let files: Vec<DirEntry> = read_dir(instances_directory)
        .expect("error while reading current directory contents")
        .map(|entry| entry.expect("error while reading a particular entry in current directory"))
        .filter(|entry| {
            entry
                .metadata()
                .expect("error while getting metadata of a particular entry in current directory")
                .is_file()
        })
        .collect();

    let g_files: Vec<&DirEntry> = files
        .iter()
        .filter(|file| {
            file.file_name()
                .to_str()
                .expect("error while getting name of a particular file in current directory")
                .starts_with("G_")
        })
        .collect();

    let common_suffixes: Vec<String> = g_files
        .iter()
        .map(|graph_file| {
            graph_file
                .file_name()
                .to_str()
                .unwrap()
                .chars()
                .skip(2)
                .collect()
        })
        .filter(|instance_suffix| !already_solved_instances.contains(instance_suffix))
        .collect();

    let reading_infos: Vec<ReadingInfo> = common_suffixes
        .iter()
        .map(|common_suffix| {
            let graph_file = "G_".to_string() + &common_suffix;
            let deps_file = "D_".to_string() + &common_suffix;
            let bounds_file = "B_".to_string() + &common_suffix;

            ReadingInfo {
                graph_file,
                deps_file,
                bounds_file,
                description: common_suffix.to_string(),
                num_workers,
            }
        })
        .collect();

    perform_multiple_benchmarks::<ReadBenchmarkImpl, _>(reading_infos.into_iter(), &output_file);
}
