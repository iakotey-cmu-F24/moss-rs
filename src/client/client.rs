use std::{
    fs::File,
    io::{self, Read, Write},
    net::{TcpStream, ToSocketAddrs},
    path::Path,
    process::exit,
};

use extend::ext;

use crate::prelude::MossConfig;

#[derive(Debug)]
pub struct MossClient<S: ToSocketAddrs> {
    server: TcpStream,
    config: MossConfig<S>,
}

impl<S: ToSocketAddrs + Default> MossClient<S> {
    pub fn new<U: ToString>(server: S, user_id: U) -> io::Result<Self> {
        MossConfig::new(user_id.to_string(), server).try_into()
    }

    pub fn send(mut self) -> String {
        self._send_headers();
        self._upload_base_files();
        self._upload_submission_files();
        self._query_server();

        self.server._read_string_512()
    }

    pub fn add_base_files<P: AsRef<str> + ToString>(&mut self, path: P, glob: bool) {
        if glob {
            self.config.add_base_file_by_glob(&path);
        } else {
            self.config.add_base_file(&path);
        }
    }

    pub fn add_files<P: AsRef<str> + ToString>(&mut self, path: P, glob: bool) {
        if glob {
            self.config.add_file_by_glob(&path);
        } else {
            self.config.add_file(&path);
        }
    }

    fn _send_file<P: AsRef<Path>>(&mut self, file_name: P, file_index: usize) {
        print!("Uploaded {:?}.... ", file_name.as_ref());
        let file_bytes: Vec<u8> = File::open(&file_name)
            .unwrap() // TODO: Return error during error handling phase
            .bytes()
            .flatten() // TODO: Return error during error handling phase
            .collect();

        let buf = file_bytes.as_slice();

        self.server.write(
            format!(
                "file {} python {} {}\n",
                file_index,
                buf.len(),
                file_name.as_ref().file_name().unwrap().to_str().unwrap() // TODO: Add error handling
            )
            .as_bytes(),
        );

        self.server.write(buf);

        println!("done.");
    }

    fn _send_headers(&mut self) {
        self.server
            .write(format!("moss {}\n", self.config.user_id()).as_bytes());
        self.server.write(
            format!(
                "directory {}\n",
                self.config.use_directory_mode().as_moss_option()
            )
            .as_bytes(),
        );
        self.server.write(
            format!(
                "X {}\n",
                self.config.use_experimental_mode().as_moss_option()
            )
            .as_bytes(),
        );
        self.server
            .write(format!("maxmatches {}\n", self.config.max_ignore_threshold()).as_bytes());
        self.server
            .write(format!("show {}\n", self.config.max_matches_displayed()).as_bytes());
        self.server
            .write(format!("language {}\n", self.config.language()).as_bytes());

        let header_response = self.server._read_string_512();

        if !header_response.trim().eq_ignore_ascii_case("yes") {
            println!("Unsupported language: {}", self.config.language());
            exit(1);
        }
    }

    fn _upload_base_files(&mut self) {
        // FIXME: Use of collect here to release mutable borrow
        // increases memory footprint. Use interiour mutability for server instead
        for file in self.config.base_files().cloned().collect::<Vec<_>>() {
            self._send_file(file, 0)
        }
    }

    fn _upload_submission_files(&mut self) {
        for (file, index) in self
            .config
            .submission_files()
            .cloned()
            .zip(1..)
            .collect::<Vec<_>>()
        {
            self._send_file(file, index as usize)
        }
    }

    fn _query_server(&mut self) {
        // FIXME: Probable problem area. Might need to manually add quotes
        self.server
            .write(format!("query 0 {}\n", self.config.comment()).as_bytes());
        println!("Query submitted.  Waiting for the server's response.\n");
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
    fn _read_string_512(&mut self) -> String {
        let mut byte_array = [32; 512];
        let bytes_read = self
            .read(&mut byte_array)
            .expect("Could not read from socket");

        String::from_utf8_lossy(&byte_array).to_string()
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
