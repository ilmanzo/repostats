use std::process::Command;
use std::io;
use rayon::prelude::*;
use chrono::{TimeZone, Utc};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct FileInfo {
    filename: String,
    commit_time: i64,
}

fn format_duration(timestamp: i64) -> String {
    let now = Utc::now();
    let commit_time = Utc.timestamp_opt(timestamp, 0).unwrap();
    let duration = now.signed_duration_since(commit_time);

    if duration.num_days() > 365 {
        format!("{} years ago", duration.num_days() / 365)
    } else if duration.num_days() > 30 {
        format!("{} months ago", duration.num_days() / 30)
    } else if duration.num_days() > 0 {
        format!("{} days ago", duration.num_days())
    } else if duration.num_hours() > 0 {
        format!("{} hours ago", duration.num_hours())
    } else if duration.num_minutes() > 0 {
        format!("{} minutes ago", duration.num_minutes())
    } else {
        "just now".to_string()
    }
}

fn main() -> io::Result<()> {
    // 1. Get the list of files
    let output = Command::new("git")
        .arg("ls-files")
        .output()?;

    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect();

    // 2. Collect file information and sort using parallel iterator
    let mut file_infos: Vec<FileInfo> = files
        .par_iter() // Changed to parallel iterator
        .filter_map(|file| {
            let output = Command::new("git")
                .args(["log", "-1", "--pretty=format:%ct", "--", file])
                .output()
                .ok()?;
            let commit_time = String::from_utf8_lossy(&output.stdout)
                .trim()
                .parse::<i64>()
                .ok()?;
            Some(FileInfo {
                filename: file.clone(),
                commit_time,
            })
        })
        .collect();

    file_infos.sort_by_key(|fi| fi.commit_time);

    // 3. Print the sorted list
    for file_info in file_infos {
        println!("{} ({})", file_info.filename, format_duration(file_info.commit_time));
    }

    Ok(())
}