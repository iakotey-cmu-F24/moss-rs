use std::path::PathBuf;

use clap::{AppSettings, ArgAction, Parser};
use libmoss::prelude::*;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
pub(crate) struct MossCliArgs {
    /// language of submitted files
    #[clap(short = 'l', long, value_parser, default_value_t = MossLanguage::C)]
    language: MossLanguage,

    /// The -d option specifies that submissions are organized by directory, not by file.
    #[clap(short = 'd', long = "directory", action = ArgAction::SetTrue, default_value_t = false)]
    use_directory_mode: bool,

    /// base (template) files for assignment
    #[clap(short = 'b', long = "base", value_parser = clap::value_parser!(PathBuf), value_name = "BASE FILES")]
    base_files: Option<Vec<PathBuf>>,

    /// maximum number of times a given passage may appear before it is ignored.
    #[clap(short = 'm', value_parser, default_value_t = 10)]
    max_match: usize,

    /// The -n option determines the number of matching files to show in the results.
    #[clap(short = 'n', value_parser, default_value_t = 250)]
    max_matches_displayed: usize,

    // ! CLI difference: Now compulsory
    /// Page 'title'
    #[clap(short = 'c', long, value_parser)]
    comment: String,

    /// Page 'title'
    #[clap(long, value_parser)]
    user_id: String,
    
    ///
    #[clap(short, long, value_parser)]
    transform: Option<String>,

    /// Use the current experimental server.
    #[clap(short = 'x', long = "experimental", action = ArgAction::SetTrue)]
    use_experimental_mode: bool,

    /// Override the name of the server.
    /// This is used for testing only
    #[clap(short, long, value_parser, default_value_t = String::from("moss.stanford.edu"))]
    server: String,

    /// Override the port.
    /// This is used for testing only
    #[clap(short, long, value_parser, default_value_t = 7690)]
    port: u16,

    /// files to check
    #[clap(value_parser = clap::value_parser!(PathBuf), value_name = "FILES")]
    submission_files: Vec<PathBuf>,
}

impl<'s> Into<MossConfig<(&'s str, u16)>> for MossCliArgs {
    fn into(self) -> MossConfig<(&'s str, u16)> {
        let server_str: &'static str = Box::leak(Box::new(self.server));
        let mut cfg: MossConfig<(&'s str, u16)> =
            MossConfig::new(self.user_id, (server_str, self.port));

        cfg.set_use_experimental_mode(self.use_experimental_mode)
            .set_max_matches_displayed(self.max_matches_displayed)
            .set_max_ignore_threshold(self.max_match)
            .set_max_matches_displayed(self.max_matches_displayed)
            .set_comment(self.comment)
            .set_language(self.language)
            .set_use_directory_mode(self.use_directory_mode)
            .set_transform(self.transform);

        if let Some(files) = self.base_files {
            files.into_iter().for_each(|file| {
                if file.exists() {
                    cfg.add_base_path(file).expect("Error occured on infallible operation!");
                } else {
                    match cfg.add_base_file(&file.to_string_lossy()) {
                        Ok(()) => (),
                        _ => todo!()
                    }
                }
            });
        }
        
        self.submission_files.into_iter().for_each(|file| {
            if file.exists() {
                cfg.add_path(file).expect("Error occured on infallible operation!");
            } else {
                match cfg.add_file(&file.to_string_lossy()) {
                    Ok(()) => (),
                    _ => todo!()
                }
            }
        });

        cfg
    }
}