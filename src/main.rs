use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    process::exit,
    str::FromStr,
    fs::File,

};

extern crate glob;
extern crate shellexpand;
use glob::glob;
use shellexpand::full;

fn main() {
    let mut sock = TcpStream::connect("moss.stanford.edu:7690").expect("Unable to connect to moss");

    let mut sock_out = String::new();

    sock.write(b"moss 123456789\n");
    sock.write(b"directory 0\n");
    sock.write(b"X 0\n");
    sock.write(b"maxmatches 10\n");
    sock.write(b"show 250\n");
    sock.write(b"language python\n");
    // sock.read_to_string(&mut sock_out);
    let out = read_byte_string(&mut sock);

    println!("Out = {out}");

    if sock_out.trim().eq_ignore_ascii_case("no") {
        println!("Unsupported language");
        exit(1);
    }


    for (idx, file) in glob(
        full("<path here>")
            .expect("could not expand path")
            .as_ref(),
    ).unwrap().enumerate() {

        if let Ok(file) = file {
            
            let file_bytes: Vec<u8> = File::open(file.clone()).unwrap().bytes().flatten().collect();
            
            let buf = file_bytes.as_slice();

            sock.write(
                format!("file {} python {} {}\n", idx+1, buf.len(), file.file_name().unwrap().to_str().unwrap())
                    .as_bytes()
            );
            
            sock.write(buf);

            println!("Uploaded {:#?}", file);

        }
    }

    sock.write(b"query 0 \"test string\"\n");

    println!("Query submitted.  Waiting for the server's response.\n");

    let result = read_byte_string(&mut sock);

    println!("{}", result);



    // }
}

fn read_byte_string(sock: &mut TcpStream) -> String {
    // It's creating a mutable array of 16 8-bit integers.
    // All integers are pre-populated as ASCII whitespace
    // let mut res = String::with_capacity(512);
    // unsafe {
    //     println!("Bytes read: {:?}", res.as_bytes());
    //     sock.read(res.as_bytes_mut());
    //     println!("Bytes read: {:?}", res.as_bytes());
    // }
    // res

    let mut byte_array = [32; 512];
    let bytes_read = sock
        .read(&mut byte_array)
        .expect("Could not read from socket");

    String::from_utf8_lossy(&byte_array).to_string()
}
