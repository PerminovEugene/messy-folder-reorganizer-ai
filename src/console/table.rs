use std::collections::HashMap;

use colored::Colorize;
use prettytable::{format, Cell, Row, Table};

use crate::{
    configuration::config::RagMlConfig, files::file_info::FilesReorganisationPlan,
    ml::hierarchical_clustering::Cluster, workflow::sources_processor::ProcessResult,
};

pub fn print_rag_processing_result(config: &RagMlConfig, process_result: &[ProcessResult]) {
    println!("{}", "📊 Files RAG processing result:".green());

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(Row::new(vec![
        Cell::new("📄 Source File Name"),
        Cell::new("📍 Closest Path"),
        Cell::new("📊 Similarity Score"),
        Cell::new("🤖 Requires LLM Reorganization"),
    ]));

    process_result.iter().for_each(|result| {
        let threshhold = config.valid_embedding_threshold.unwrap();
        let need_reorganize = if result.score < threshhold {
            "Yes"
        } else {
            "No"
        };
        table.add_row(Row::new(vec![
            Cell::new(&result.source_file_name),
            Cell::new(&result.destination_relative_path),
            Cell::new(&result.score.to_string()),
            Cell::new(need_reorganize),
        ]));
    });

    table.printstd();
}

pub fn print_clustering_table(clusters: &[Cluster], pathes: &[String]) {
    println!("{}", "🗂️ Files clustering result:".green());

    let mut table: Table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(Row::new(vec![
        Cell::new("🔢 Cluster Number"),
        Cell::new("📄 File name"),
    ]));

    clusters.iter().for_each(|cluster| {
        for &member in &cluster.members {
            table.add_row(Row::new(vec![
                Cell::new(cluster.id.to_string().as_str()),
                Cell::new(&pathes[member]),
            ]));
        }
    });

    table.printstd();
}

pub fn print_clusters_ai_proposed_names(folder_data: &HashMap<usize, String>) {
    println!("{}", "🗂️ Ai generated folder names for clusters:".green());

    let mut table: Table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(Row::new(vec![
        Cell::new("🔢 Cluster Number"),
        Cell::new("📄 Folder name"),
    ]));

    folder_data
        .iter()
        .for_each(|(cluster_number, folder_name)| {
            table.add_row(Row::new(vec![
                Cell::new(cluster_number.to_string().as_str()),
                Cell::new(folder_name),
            ]));
        });

    table.printstd();
}

pub fn print_migration_plan_table(files_reorganization_plan: &[FilesReorganisationPlan]) {
    println!("{}", "🚚 Files migration plan:".green());

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    files_reorganization_plan.first().iter().for_each(|plan| {
        let from = format!("📤 From: {}", plan.source);
        let to = format!("📥 To: {}", plan.destination);

        table.set_titles(Row::new(vec![Cell::new(&from), Cell::new(&to)]));
    });

    files_reorganization_plan.iter().for_each(|plan| {
        // separators are needed for case when inner_path == "./" to avoid double slash
        let source_separator = if plan.source_inner_path == "./" {
            ""
        } else {
            "/"
        };
        let destination_separator = if plan.destination_inner_path == "./" {
            ""
        } else {
            "/"
        };
        let from = format!(
            "{}{}{}",
            plan.source_inner_path, source_separator, plan.file_name
        );
        let to = format!(
            "{}{}{}",
            plan.destination_inner_path, destination_separator, plan.file_name
        );

        table.add_row(Row::new(vec![Cell::new(&from), Cell::new(&to)]));
    });

    table.printstd();
}
