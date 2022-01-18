use anyhow::Result;
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use tera::{Context, Tera};

const APP_START: &str = "<!-- start applications -->";
const APP_END: &str = "<!-- end applications -->";
const LIB_START: &str = "<!-- start libraries -->";
const LIB_END: &str = "<!-- end libraries -->";
const MEETUP_START: &str = "<!-- start meetups -->";
const MEETUP_END: &str = "<!-- end meetups -->";

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Data {
    // These fields are purposefully ordered for `data.json`.
    pub applications: Option<Vec<Project>>,
    pub libraries: Option<Vec<Project>>,
    pub meetups: Option<Vec<Meetup>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Project {
    // These fields are purposefully ordered for `data.json`.
    pub title: String,
    pub url: String,
    pub description: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Meetup {
    pub id: u32,
    pub date: NaiveDate,
    pub region: String,
    pub cet_time: String,
    pub pdt_time: String,
    pub agenda: Option<Vec<AgendaItem>>,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct AgendaItem {
    pub title: String,
    pub url: Option<String>,
    pub description: String,
    pub speaker_name: Option<String>,
    pub speaker_url: Option<String>,
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

fn process_meetup(meetup: &Meetup) -> Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let meetup_path = root
        .join("docs")
        .join("meetup")
        .join(format!("{}.md", meetup.date.to_string()));
    let meetup_writer = fs::File::create(meetup_path)?;

    let tera = Tera::new("templates/**/*.md")?;
    let mut context = Context::new();
    context.insert("meetup", &meetup);
    tera.render_to("meetup.md", &context, meetup_writer)?;

    Ok(())
}

fn process_meetups(file_writer: &mut fs::File, meetups: &[Meetup]) -> Result<()> {
    let today = Utc::today().naive_utc();
    let mut upcoming = true;
    for meetup in meetups {
        if upcoming && meetup.date < today {
            writeln!(file_writer, "\n#### Past\n")?;
            upcoming = false;
        }
        writeln!(
            file_writer,
            "- **[Rust Cloud Native online meetup #{}](meetup/{}.md)** ({})",
            meetup.id, meetup.date, meetup.date,
        )?;

        process_meetup(meetup)?;
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
        meetups: match data.meetups.clone() {
            Some(mut s) => {
                s.sort_by(|a, b| b.cmp(a));
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
                if line == APP_END || line == LIB_END || line == MEETUP_END {
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
                } else if line == MEETUP_START {
                    if let Some(meetups) = &formatted_data.meetups {
                        process_meetups(&mut index_writer, meetups)?;
                    }
                    locked = true;
                }
            }
        }
    }
    Ok(())
}
