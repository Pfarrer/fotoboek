use std::path::Path;

use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(name = "Family Album", setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(short, long)]
    pub image_root: String,

    #[clap(short, long)]
    pub data_root: String,
}

pub fn parse_args() -> Result<Args, String> {
    let args = Args::parse();

    validate_args(&args)?;
    Result::Ok(args)
}

fn validate_args(args: &Args) -> Result<(), String> {
    if !Path::new(&args.image_root).is_dir() {
        return Result::Err("image_root is not a directory!".to_string())
    }

    Result::Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absolute_image_root_and_data_root_given() {
        let result = Args::try_parse_from(&[
            "test",
            "--image-root",
            "/home/me",
            "--data-root",
            "/tmp/repo"
        ]);

        assert!(result.is_ok());
        
        let args = result.ok().unwrap();
        assert_eq!(args.image_root, "/home/me");
        assert_eq!(args.data_root, "/tmp/repo");
    }

    #[test]
    fn no_args_given() {
        let result = Args::try_parse_from(&[
            "test"
        ]);

        assert!(result.is_err());
    }
}