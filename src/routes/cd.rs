use std::{collections::HashMap, process::Command};

use serde::Deserialize;
use actix_web::{web, get, HttpResponse, Responder};

#[derive(Debug, Deserialize)]
struct BuildConfig {
    tasks: HashMap<String, BuildTask>,
}

#[derive(Debug, Deserialize)]
struct BuildTask {
    command: String,
    args: Vec<String>,
}

#[get("/cd/{task_name}")]
pub async fn cd(path: web::Path<String>) -> impl Responder {
    let config_str = std::fs::read_to_string("build.toml").unwrap();
    let config: BuildConfig = toml::from_str(&config_str).unwrap();

    let task = config.tasks.get(path.as_ref()).unwrap();

    Command::new(&task.command)
        .args(&task.args)
        .spawn()
        .expect("failed to execute process");

    HttpResponse::Ok().body(format!("{:#?}", task))
}
