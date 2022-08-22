use libmoss::prelude::*;

fn main() {
    let mut test_client =
        MossClient::new("moss.stanford.edu:7690", "<User ID here>").expect("Unable to connect to moss");

    test_client.add_files(
        r"<Test path here>",
        true,
    );

    let result = test_client.send();
    println!("{}", result);

    // }
}
