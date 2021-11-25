use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;

const APP_START: &str = "<!-- start applications -->";
const APP_END: &str = "<!-- end applications -->";
const LIB_START: &str = "<!-- start libraries -->";
const LIB_END: &str = "<!-- end libraries -->";

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Data {
    // These fields are purposefully ordered for `data.json`.
    pub applications: Option<Vec<Project>>,
    pub libraries: Option<Vec<Project>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Project {
    // These fields are purposefully ordered for `data.json`.
    pub title: String,
    pub url: String,
    pub description: String,
}

fn process_projects(file_writer: &mut fs::File, projects: &[Project]) -> Result<()> {
    for project in projects {
        writeln!(
            file_writer,
            "- **[{}]({})**: {}",
            project.title, project.url, project.description
        )?;
    }
    Ok(())
}

fn format_data_json(data: &Data) -> Data {
    Data {
        applications: match data.applications.clone() {
            Some(mut s) => {
                s.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
                Some(s)
            }
            None => None,
        },
        libraries: match data.libraries.clone() {
            Some(mut s) => {
                s.sort_by(|a, b| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
                Some(s)
            }
            None => None,
        },
    }
}

fn main() -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));

    let data_path = root.join("data.json");
    let data: Data = serde_json::from_str(&fs::read_to_string(&data_path)?)?;
    let formatted_data = format_data_json(&data);

    {
        // Include `data.json` writer in its own scope. We want to close the file before continuing.
        serde_json::to_writer_pretty(fs::File::create(data_path)?, &formatted_data)?;
    }

    let index_path = root.join("docs").join("index.md");
    let index_contents = fs::read_to_string(&index_path)?;
    let mut index_writer = fs::File::create(index_path)?;

    let mut locked = false;
    for line in index_contents.lines() {
        match locked {
            true => {
                if line == APP_END || line == LIB_END {
                    writeln!(&index_writer, "{}", line)?;
                    locked = false;
                }
            }
            false => {
                writeln!(&index_writer, "{}", line)?;
                if line == APP_START {
                    if let Some(applications) = &formatted_data.applications {
                        process_projects(&mut index_writer, applications)?;
                    }
                    locked = true;
                } else if line == LIB_START {
                    if let Some(libraries) = &formatted_data.libraries {
                        process_projects(&mut index_writer, libraries)?;
                    }
                    locked = true;
                }
            }
        }
    }
    Ok(())
}
