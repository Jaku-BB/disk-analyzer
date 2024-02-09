mod parser_data;

use crate::parser_data::ParserData;
use clap::Parser;
use std::fs::{metadata, read_dir};
use std::path::Path;

fn main() {
    let parser_data = ParserData::parse();
    let path = Path::new(&parser_data.path);

    if !path.exists() || path.is_file() {
        panic!("The path provided does not exist, is not a directory, or is inaccessible.");
    }

    let (directory_count, file_count, total_size) = traverse_entry(
        &path,
        parser_data.recursive,
        parser_data.depth,
        parser_data.human_unit,
        parser_data.quiet,
        parser_data.ignore_extension,
    );

    println!(
        "Directory count: {}\nFile count: {}\nTotal size {}: {}",
        directory_count,
        file_count,
        if !parser_data.recursive {
            "(only top level, use --recursive to include any directory within the path)"
        } else {
            ""
        },
        if parser_data.human_unit {
            convert_size_to_human_unit(total_size)
        } else {
            total_size.to_string()
        }
    );
}

fn traverse_entry(
    path: &Path,
    recursive: bool,
    depth: Option<usize>,
    human_unit: bool,
    quiet: bool,
    ignore_extension: Option<Vec<String>>,
) -> (u32, u32, u64) {
    fn traverse_entry_recursive(
        path: &Path,
        recursive: bool,
        depth: Option<usize>,
        current_depth: u32,
        human_unit: bool,
        quiet: bool,
        ignore_extension: Option<Vec<String>>,
    ) -> (u32, u32, u64) {
        let mut directory_count = 0;
        let mut file_count = 0;
        let mut total_size = 0;

        if let Some(maximum_depth) = depth {
            if current_depth > maximum_depth as u32 {
                return (directory_count, file_count, total_size);
            }
        }

        match read_dir(path) {
            Ok(content) => {
                let entries: Vec<_> = content
                    .filter_map(Result::ok)
                    .map(|entry| (entry.path(), entry.file_type().ok()))
                    .collect();

                for (entry_path, entry_type) in entries {
                    match entry_type {
                        Some(file_type) if file_type.is_file() => {
                            let extension = entry_path.extension();
                            if let Some(extension) = extension {
                                if let Some(extension_to_ignore) = &ignore_extension {
                                    if extension_to_ignore.contains(
                                        &extension.to_string_lossy().to_lowercase(),
                                    ) {
                                        continue;
                                    }
                                }
                            }

                            let size = match metadata(&entry_path) {
                                Ok(data) => data.len(),
                                Err(_) => {
                                    println!(
                                        "{}\nCould not get entry data, is it accessible? Skipping...",
                                        entry_path.display()
                                    );
                                    continue;
                                }
                            };

                            if !quiet {
                                print_entry_data(
                                    current_depth as usize,
                                    &entry_path.file_name().unwrap().to_string_lossy(),
                                    true,
                                    size,
                                    human_unit,
                                );
                            }

                            file_count += 1;
                            total_size += size;
                        }
                        Some(file_type) if file_type.is_dir() => {
                            if !quiet {
                                print_entry_data(
                                    current_depth as usize,
                                    &entry_path.file_name().unwrap().to_string_lossy(),
                                    false,
                                    0,
                                    human_unit,
                                );
                            }

                            directory_count += 1;

                            if recursive {
                                let (sub_dir_count, sub_file_count, sub_total_size) =
                                    traverse_entry_recursive(
                                        &entry_path,
                                        recursive,
                                        depth,
                                        current_depth + 1,
                                        human_unit,
                                        quiet,
                                        ignore_extension.clone(),
                                    );

                                directory_count += sub_dir_count;
                                file_count += sub_file_count;
                                total_size += sub_total_size;
                            }
                        }
                        _ => {
                            println!(
                                "{}\nCould not read entry, is it accessible? Skipping...",
                                path.display()
                            );
                        }
                    }
                }
            }
            Err(_) => {
                println!(
                    "{}\nCould not read directory content, is it accessible? Skipping...",
                    path.display()
                );
            }
        }

        (directory_count, file_count, total_size)
    }

    traverse_entry_recursive(
        path,
        recursive,
        depth,
        0,
        human_unit,
        quiet,
        ignore_extension,
    )
}

fn print_entry_data(
    current_depth: usize,
    entry_name: &str,
    is_file: bool,
    size: u64,
    human_unit: bool,
) {
    let indent = "---> ".repeat(current_depth);

    let size = if is_file {
        format!(
            "| size: {}",
            if human_unit {
                convert_size_to_human_unit(size)
            } else {
                size.to_string()
            }
        )
    } else {
        "".to_string()
    };

    println!("{}{} {}", indent, entry_name, size);
}

fn convert_size_to_human_unit(size: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} [{}]", size, units[unit_index])
}
