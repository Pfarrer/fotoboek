use std::path::PathBuf;
use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(name = "Family Album", setting = AppSettings::ColoredHelp)]
pub struct Args {
    #[clap(short, long)]
    pub image_root: PathBuf,

    #[clap(short, long)]
    pub data_root: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
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
        assert_eq!(args.image_root.to_str(), Option::Some("/home/me"));
        assert_eq!(args.data_root.to_str(), Option::Some("/tmp/repo"));
    }

    #[test]
    fn no_args_given() {
        let result = Args::try_parse_from(&[
            "test"
        ]);

        assert!(result.is_err());
    }
}