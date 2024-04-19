use git2::Repository;
use serde::{Deserialize, Serialize};

fn main() {
    let projects_path = "B:/etc/projects/";
    let out_file = "./projects.toml";
    for entry in std::fs::read_dir(projects_path).unwrap() {
        let entry = entry.unwrap().path();

        let cargo_toml = entry.join("Cargo.toml");

        if cargo_toml.exists() {
            let Ok(project) = std::fs::read_to_string(entry.join("Cargo.toml")) else { continue; };
            let Ok(project) = toml::from_str::<CargoProject>(&project) else { continue; };
            let authors = project.package.authors.join(",").to_lowercase();
            
            if authors.contains(&"hoovy".to_string()) || authors.contains(&"marco".to_string()) {

                let src_status = if project.package.homepage.is_some() { SourceStatus::Open } else { SourceStatus::Closed };

                println!("{entry:?}");

                if let Ok(repo) = Repository::open(".") {
                    println!("{}",repo.message().unwrap());
                };

                let item = BuildConfigProject {
                    name: project.package.name,
                    description: project.package.description,
                    source_status: src_status,
                    src: project.package.homepage,
                    dev_status: todo!(),
                    license: todo!(),
                    maintenance: todo!(),
                    languages: todo!(),
                    tools: todo!(),
                };
            }

        }
        
    }
}

/// The Config used in building the website
#[derive(Debug, Serialize, Deserialize)]
pub struct BuildConfig {
    pub projects: Vec<BuildConfigProject>,
}

impl BuildConfig {
    pub fn add_project(&mut self, project: BuildConfigProject)  {
        if self.projects.clone().into_iter().any(|p| p.name == project.name) {
            println!("Project already exists");
        }
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
    license: String,
    maintenance: MaintenanceStatus,
    languages: Vec<String>,
    tools: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SourceStatus {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "open")]
    Open,
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
