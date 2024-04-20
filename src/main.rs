use std::{fs, time::{SystemTime, UNIX_EPOCH}, env};

use git2::Repository;
use serde::{Deserialize, Serialize};

#[allow(unused_assignments)]
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <work folder> <out-file> [input-file]", args[0]);
        return;
    }

    let out_file = &args[2];
    let projects_path = &args[1];

    let mut build_config = match fs::read_to_string(args.get(3).map(String::as_str).unwrap_or("./projects.toml")) {
        Ok(file) => { toml::from_str::<BuildConfig>(&file).unwrap() },
        Err(_) => {
            BuildConfig {
                project: vec![],
            }
        }
    };

    for entry in std::fs::read_dir(projects_path).unwrap() {
        let entry = entry.unwrap().path();

        let cargo_toml = entry.join("Cargo.toml");

        if cargo_toml.exists() {
            let Ok(project) = std::fs::read_to_string(entry.join("Cargo.toml")) else { continue; };
            let Ok(project) = toml::from_str::<CargoProject>(&project) else { continue; };
            let authors = project.package.authors.join(",").to_lowercase();
            
            if authors.contains(&"hoovy".to_string()) || authors.contains(&"marco".to_string()) {

                let src_status = if project.package.homepage.is_some() { SourceStatus::Open } else { SourceStatus::Closed };

                let mut last_commit_time = 0;
                let mut first_commit_time = 0;

                if let Ok(repo) = Repository::open(entry.clone()) {
                    if let Ok(head) = repo.head() { 
                        if let Ok(commit) = head.peel_to_commit() {

                            if let Some(parent) = commit.parents().last() {
                                first_commit_time = parent.time().seconds();
                            }
        
                            last_commit_time = commit.time().seconds();
                        };
                    } else { println!("No head! Please Commit! ({entry:?})"); };
                };

                let start = SystemTime::now();
                let current_epoch = start
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards");

                let current_epoch = current_epoch.as_secs() as i64;

                let time_since = current_epoch - last_commit_time;

                let new = (current_epoch - first_commit_time) < 1_555_200; // 18 days

                let maintance_status = if time_since > 2592000 { // 30 days
                    if time_since > 7776000 { MaintenanceStatus::Deprecated } else { MaintenanceStatus::Maintenance } // 90 days
                } else { MaintenanceStatus::Active };

                let item = BuildConfigProject {
                    name: project.package.name,
                    description: project.package.description,
                    source_status: src_status,
                    src: project.package.homepage,
                    dev_status: DevelopmentStatus::Stable,
                    license: None,
                    maintenance: maintance_status,
                    languages: vec!["rust".to_string()], // TODO: Make this dynamic
                    tools: vec![],
                    epoch: last_commit_time,
                    new: new,
                };

                build_config.add_project(item);
            }
        }
    }

    fs::write(out_file, toml::to_string_pretty(&build_config).unwrap()).unwrap();

}

/// The Config used in building the website
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig { pub project: Vec<BuildConfigProject> }

impl BuildConfig {
    pub fn add_project(&mut self, project: BuildConfigProject) {
        for item in self.project.iter_mut() {
            let p2 = project.clone();
            if item.name.replace("-", " ") == p2.name.replace("-", " ") {

                item.description = p2.description;
                item.source_status = p2.source_status;
                item.src = p2.src;
                item.license = p2.license;
                item.maintenance = p2.maintenance;
                item.epoch = p2.epoch;
                item.new = p2.new;

                return;
            }
        }

        self.project.push(project);
    }
}

/// The Config used in building the website
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuildConfigProject {
    name: String,
    description: String,
    source_status: SourceStatus,
    src: Option<String>,
    dev_status: DevelopmentStatus,
    license: Option<String>,
    maintenance: MaintenanceStatus,
    languages: Vec<String>,
    tools: Vec<String>,
    epoch: i64,
    new: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SourceStatus {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "planned")]
    Planned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MaintenanceStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deprecated")]
    Deprecated,
    #[serde(rename = "maintenance")]
    Maintenance
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DevelopmentStatus {
    #[serde(rename = "stable")]
    Stable,
    #[serde(rename = "beta")]
    Beta,
    #[serde(rename = "alpha")]
    Alpha,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoProject {
    pub package: CargoPackage,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<String>,
    pub homepage: Option<String>,
}
