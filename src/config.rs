use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::types::Position;
use crate::{common, constants};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub imagepath: Option<String>,
    pub outdir: Option<String>,
    pub outimage: Option<String>,
    pub position: Option<Position>,
    pub size: Option<u32>,
    pub expand: Option<bool>,
    #[serde(skip)]
    pub config: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            imagepath: None,
            outdir: Some(String::new()),
            outimage: Some(String::new()),
            position: Some(Position::Start),
            size: Some(constants::DEFAULT_SIZE),
            expand: Some(false),
            config: Self::get_default_config_path()
                .map(|path| {
                    path.to_string_lossy().to_string()
                }),
        }
    }
}

impl Config {
    pub fn load_config(filename: &str) -> Result<Config> {
        let file = File::open(filename)?;
        let mut conf: Config = serde_json::from_reader(BufReader::new(file))?;
        conf.config = Some(filename.to_string());
        Ok(conf)
    }

    pub fn load_default_config() -> Result<Config> {
        let default_config_path = Self::get_default_config_path()
            .context("デフォルト設定ファイルのパスを取得できません")?;
        let file = File::open(&default_config_path)?;
        Ok(serde_json::from_reader(BufReader::new(file))?)
    }

    pub fn create_if_not_exists(path: &Path) -> Result<Config, std::io::Error> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;
        let config = Config::default();
        serde_json::to_writer_pretty(BufWriter::new(file), &config)?;
        Ok(config)
    }

    pub fn get_default_config_path() -> Option<PathBuf> {
        Some(common::get_module_dir()?.join(constants::DEFAULT_CONFIG_FILE))
    }
}
