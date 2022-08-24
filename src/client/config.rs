use std::{net::ToSocketAddrs, path::PathBuf, str::FromStr};

use getset_scoped::{Getters, Setters};

use crate::prelude::MossLanguage;

#[derive(Debug, Default, Getters, Setters)]
#[getset(get = "pub", set = "pub")]
#[allow(dead_code)] // TODO: Remove
pub struct MossConfig<S: ToSocketAddrs> {
    #[getset(get = "pub")]
    server_address: S,

    user_id: String,

    comment: String,
    language: MossLanguage,
    use_directory_mode: bool,

    use_experimental_mode: bool,
    max_matches_displayed: usize,
    max_ignore_threshold: usize,

    #[getset(skip)]
    _base_files: Vec<PathBuf>,

    #[getset(skip)]
    _submission_files: Vec<PathBuf>,
}

impl<S: ToSocketAddrs> MossConfig<S> {
    pub fn new<U: ToString>(user_id: U, server_address: S) -> Self {
        MossConfig {
            server_address,
            user_id: user_id.to_string(),
            use_experimental_mode: Default::default(),
            max_matches_displayed: 250,
            max_ignore_threshold: 10,
            _base_files: Default::default(),
            _submission_files: Default::default(),
            comment: Default::default(),
            language: Default::default(),
            use_directory_mode: Default::default(),
        }
    }

    pub fn add_base_file<P: AsRef<str> + ToString>(&mut self, path: &P) -> &mut Self {
        if let Ok(p) = PathBuf::from_str(path.as_ref()) {
            // infallible operation.
            if p.exists() {
                self._base_files.push(p);
            } else {
                let full_path: String =
                    shellexpand::full(path).map_or(path.to_string(), |x| x.into_owned()); // failure cause: unable to expand path
                let matches = glob::glob(&full_path).unwrap(); // Failure cause: pattern error
                matches
                    .inspect(|x| ()) // log inaccessible paths here
                    .filter_map(Result::ok)
                    .for_each(|p| self._base_files.push(p));
            }
        }

        self
    }

    pub fn add_file<P: AsRef<str> + ToString>(&mut self, path: &P) -> &mut Self {
        if let Ok(p) = PathBuf::from_str(path.as_ref()) {
            // infallible operation.
            if p.exists() {
                self._submission_files.push(p);
            } else {
                let full_path: String =
                    shellexpand::full(path).map_or(path.to_string(), |x| x.into_owned()); // failure cause: unable to expand path
                let matches = glob::glob(&full_path).unwrap(); // Failure cause: pattern error
                matches
                    .inspect(|x| ()) // log inaccessible paths here
                    .filter_map(Result::ok)
                    .for_each(|p| self._submission_files.push(p));
            }
        }

        self
    }

    pub fn add_path(&mut self, path: PathBuf) -> &mut Self {
        self._submission_files.push(path);
        self
    }

    pub fn add_base_path(&mut self, path: PathBuf) -> &mut Self {
        self._base_files.push(path);
        self
    }

    pub fn base_files(&self) -> impl Iterator<Item = &PathBuf> + '_ {
        self._base_files.iter()
    }

    pub fn submission_files<'a>(&'a self) -> impl Iterator<Item = &PathBuf> + 'a {
        self._submission_files.iter()
    }
}
