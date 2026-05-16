use crate::args::Args;
use crate::common;
use crate::config::Config;
use crate::constants;
use crate::types::Position;
use anyhow::{Result, bail};
use clap::Parser;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Params {
    pub imagepath: String,
    pub outdir: String,
    pub outimage: String,
    pub position: Position,
    pub size: u32,
    pub expand: bool,
    pub config: String,
}

impl From<&Config> for Params {
    fn from(v: &Config) -> Self {
        let default_path =
            Config::get_default_config_path().map(|path| path.to_string_lossy().to_string());
        Params {
            imagepath: v.imagepath.as_deref().unwrap_or_default().to_string(),
            outdir: v.outdir.as_deref().unwrap_or_default().to_string(),
            outimage: v.outimage.as_deref().unwrap_or_default().to_string(),
            position: v.position.unwrap_or(Position::Start),
            size: v.size.unwrap_or(constants::DEFAULT_SIZE),
            expand: v.expand.unwrap_or_default(),
            config: v
                .config
                .as_deref()
                .unwrap_or(&default_path.unwrap_or_default())
                .to_string(),
        }
    }
}

impl Params {
    pub fn get_params() -> Result<Params> {
        let args = Args::parse();
        let config: Config = if let Some(filename) = args.config.as_deref() {
            Config::load_config(filename)?
        } else {
            eprintln!("デフォルトの設定ファイルを使用します");
            match Config::create_if_not_exists(&Config::get_default_config_path().ok_or(
                std::io::Error::new(std::io::ErrorKind::NotFound, "パスを取得できません"),
            )?) {
                Ok(conf) => conf,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    Config::load_default_config()?
                }
                Err(e) => return Err(e.into()),
            }
        };
        let params = Self::resolve_params(&args, &config);
        if params.imagepath.is_empty() {
            bail!("画像ファイルが指定されていないです")
        } else if params.outdir.is_empty() {
            bail!("出力先のフォルダーを特定できません")
        }
        Ok(params)
    }

    fn resolve_params(args: &Args, conf: &Config) -> Params {
        let mut params: Params = conf.into();
        params.imagepath = args
            .imagepath
            .as_deref()
            .unwrap_or(&params.imagepath)
            .to_string();
        params.outdir = args.outdir.as_deref().unwrap_or(&params.outdir).to_string();
        params.outimage = args
            .outimage
            .as_deref()
            .unwrap_or(&params.outimage)
            .to_string();
        params.position = args.position.unwrap_or(params.position);
        params.size = args.size.unwrap_or(params.size);
        params.expand = args.expand.unwrap_or(params.expand);
        params.config = args.config.as_deref().unwrap_or(&params.config).to_string();

        if params.outdir.is_empty() {
            params.outdir = common::get_path_str(
                &common::get_parent_dir(&PathBuf::from(&params.imagepath)).unwrap_or_default(),
            );
        }
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_values_override_config() {
        let args = Args {
            imagepath: Some("cli.png".into()),
            outdir: Some("cli_out".into()),
            outimage: None,
            position: Some(Position::End),
            size: Some(512),
            expand: Some(true),
            config: None,
        };

        let conf = Config {
            imagepath: Some("conf.png".into()),
            outdir: Some("conf_out".into()),
            outimage: Some("conf.jpg".into()),
            position: Some(Position::Start),
            size: Some(constants::DEFAULT_SIZE),
            expand: Some(false),
            config: Some("imgsquare.conf".into()),
        };

        let params = Params::resolve_params(&args, &conf);

        assert_eq!(params.imagepath, "cli.png");
        assert_eq!(params.outdir, "cli_out");
        assert_eq!(params.position, Position::End);
        assert_eq!(params.size, 512);
        assert!(params.expand);
    }

    #[test]
    fn config_values_are_used_when_args_missing() {
        let args = Args {
            imagepath: None,
            outdir: None,
            outimage: None,
            position: None,
            size: None,
            expand: None,
            config: None,
        };

        let conf = Config {
            imagepath: Some("conf.png".into()),
            outdir: Some("conf_out".into()),
            outimage: Some("conf.jpg".into()),
            position: Some(Position::Center),
            size: Some(1024),
            expand: Some(true),
            config: Some("imgsquare.conf".into()),
        };

        let params = Params::resolve_params(&args, &conf);

        assert_eq!(params.imagepath, "conf.png");
        assert_eq!(params.outdir, "conf_out");
        assert_eq!(params.position, Position::Center);
        assert_eq!(params.size, 1024);
        assert!(params.expand);
    }
}
