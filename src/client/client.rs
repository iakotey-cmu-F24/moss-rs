use std::{
    fs::File,
    io::{self, BufReader, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::Path,
    process::exit,
};

use extend::ext;
use regex::Regex;
use snafu::{prelude::*, Whatever};

use crate::prelude::MossConfig;

#[derive(Debug)]
pub struct MossClient<S: ToSocketAddrs> {
    server: TcpStream,
    config: MossConfig<S>,
}

impl<S: ToSocketAddrs> MossClient<S> {
    pub fn new<U: ToString>(server: S, user_id: U) -> io::Result<Self> {
        MossConfig::new(user_id.to_string(), server).try_into()
    }

    pub fn send(mut self) -> Result<String, Whatever> {
        self._send_headers()?;
        self._upload_base_files()?;
        self._upload_submission_files()?;
        self._query_server()?;

        self.server
            ._read_string_512()
            .whatever_context("Unable to read Url from server")
    }

    pub fn add_base_file<P: AsRef<str> + ToString>(&mut self, path: P) {
        self.config.add_base_file(&path);
    }

    pub fn add_file<P: AsRef<str> + ToString>(&mut self, path: P) {
        self.config.add_file(&path);
    }

    fn _send_file<P: AsRef<Path>>(
        &mut self,
        file: P,
        file_index: usize,
        display_name: Option<&dyn AsRef<str>>,
    ) -> Result<(), Whatever> {
        if file.as_ref().exists() {
            print!("Uploaded {:?}.... ", file.as_ref());

            let f = File::open(&file).whatever_context("Could not open file")?;
            let mut file_buffer = Vec::new();

            let _ = BufReader::new(f)
                .read_to_end(&mut file_buffer)
                .whatever_context("Could not read data from file")?;

            let file_name = file
                .as_ref()
                .file_name()
                .map_or(None, |os_str| os_str.to_str())
                .whatever_context("Invalid / Non UTF-8 file name")?;


            let display_name = match display_name {
                Some(pattern) => {
                    let re = Regex::new(pattern.as_ref())
                        .whatever_context("Invalid regex expression provided")?;

                    re.captures(file_name)
                        .and_then(|c| c.get(0).map(|f| f.as_str()))
                        .unwrap_or(file_name)
                }
                None => file_name,
            }
            .replace(" ", "_");

            self.server
                .write(
                    format!(
                        "file {} python {} {}\n",
                        file_index,
                        self.config.language(),
                        file_buffer.len(),
                        display_name.replace(" ", "_")
                    )
                    .as_bytes(),
                )
                .whatever_context("Unable to write file description")?;

            self.server
                .write(file_buffer.as_slice())
                .whatever_context("Unable to write file description")?;

            println!("done.");
            Ok(())
        } else {
            whatever!("File does not exist")
        }
    }

    fn _send_headers(&mut self) -> Result<(), Whatever> {
        let _ = self
            .server
            .write(format!("moss {}\n", self.config.user_id()).as_bytes())
            .whatever_context("Could not authenticate with Moss")?;
        let _ = self
            .server
            .write(
                format!(
                    "directory {}\n",
                    self.config.use_directory_mode().as_moss_option()
                )
                .as_bytes(),
            )
            .whatever_context("Error sending directory information to Moss server")?;
        let _ = self
            .server
            .write(
                format!(
                    "X {}\n",
                    self.config.use_experimental_mode().as_moss_option()
                )
                .as_bytes(),
            )
            .whatever_context("Error sending experimental information to Moss server")?;
        let _ = self
            .server
            .write(format!("maxmatches {}\n", self.config.max_ignore_threshold()).as_bytes())
            .whatever_context("Error sending match information to Moss server")?;
        let _ = self
            .server
            .write(format!("show {}\n", self.config.max_matches_displayed()).as_bytes())
            .whatever_context("Error sending display information to Moss server")?;
        let _ = self
            .server
            .write(format!("language {}\n", self.config.language()).as_bytes())
            .whatever_context("Error sending language information to Moss server")?;

        let header_response = self
            .server
            ._read_string_512()
            .whatever_context("Unable to recieve server's header response")?;

        if !header_response.trim().eq_ignore_ascii_case("yes") {
            println!("Unsupported language: {}", self.config.language());
            exit(1);
        }

        Ok(())
    }

    fn _upload_base_files(&mut self) -> Result<(), Whatever> {
        // FIXME: Use of collect here to release mutable borrow
        // increases memory footprint. Use interior mutability for server instead
        for file in self.config.base_files().cloned().collect::<Vec<_>>() {
            self._send_file(file, 0, None)?;
        }
        Ok(())
    }

    fn _upload_submission_files(&mut self) -> Result<(), Whatever> {
        for (file, index) in self
            .config
            .submission_files()
            .cloned()
            .zip(1..)
            .collect::<Vec<_>>()
        {
            self._send_file(file, index as usize, None)?;
        }
        Ok(())
    }

    fn _query_server(&mut self) -> Result<(), Whatever> {
        // FIXME: Probable problem area. Might need to manually add quotes
        self.server
            .write(format!("query 0 {}\n", self.config.comment()).as_bytes())
            .whatever_context("Could not send query to server")?;
        println!("Query submitted.  Waiting for the server's response.\n");
        Ok(())
    }
}

impl<S: ToSocketAddrs> TryFrom<MossConfig<S>> for MossClient<S> {
    type Error = io::Error;
    fn try_from(config: MossConfig<S>) -> Result<Self, Self::Error> {
        Ok(MossClient {
            server: TcpStream::connect(&config.server_address())?,
            config,
        })
    }
}

#[doc(hidden)]
#[ext]
impl TcpStream {
    /// Read a string from the socket using a 512 byte buffer
    /// This method is for internal use only.
    fn _read_string_512(&mut self) -> Result<String, io::Error> {
        let mut byte_array = [32; 512];

        match self.read(&mut byte_array) {
            Ok(_bytes_read) => Ok(String::from_utf8_lossy(&byte_array).to_string()),
            Err(err) if err.kind() == io::ErrorKind::Interrupted => Ok(String::new()),
            Err(err) => Err(err),
        }
    }
}

#[doc(hidden)]
#[ext]
impl bool {
    #[inline(always)]
    fn as_moss_option(&self) -> &'static str {
        if *self == true {
            "1"
        } else {
            "0"
        }
    }
}
