use std::{
    collections::HashSet,
    fs::{read_dir, DirEntry, File},
    io::{BufRead, BufReader},
};

use instance_bb_solver::{ReadBenchmarkImpl, ReadingInfo};
use solving_manager::utils::perform_multiple_benchmarks;

fn main() {
    let num_workers = 4;

    let output_file = "parallel_branch_bound".to_string();

    let files: Vec<DirEntry> = read_dir("./")
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

    let reading_infos: Vec<ReadingInfo> = g_files
        .iter()
        .map(|graph_file| -> ReadingInfo {
            let common_suffix: String = graph_file
                .file_name()
                .to_str()
                .unwrap()
                .chars()
                .skip(2)
                .collect();

            let graph_file = "G_".to_string() + &common_suffix;
            let deps_file = "D_".to_string() + &common_suffix;
            let bounds_file = "B_".to_string() + &common_suffix;

            ReadingInfo {
                graph_file,
                deps_file,
                bounds_file,
                description: common_suffix,
                num_workers,
            }
        }).collect();

    perform_multiple_benchmarks::<ReadBenchmarkImpl, _>(reading_infos.into_iter(), &output_file);
}
