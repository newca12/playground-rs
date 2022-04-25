use libvips::{ops, VipsApp, VipsImage};

pub mod global {
    use std::io::{self, Write};

    static FILENAME: &str = "sample";
    pub static WIDTH: i32 = 110;

    const OCCURENCES: &[(&str, u32)] = &[
        ("5b", 1000),
        ("5", 1000),
        ("10b", 1000),
        ("10", 1000),
        ("20", 1000),
        ("30", 1000),
        ("40b", 1000),
        ("40", 1000),
        ("50", 1000),
        ("60", 500),
        ("70", 400),
        ("80", 300),
        ("90", 200),
        ("100", 100),
    ];

    #[derive(Clone, Hash, Debug, PartialEq)]
    pub struct Test {
        pub name: String,
        pub quality: String,
        pub origin: String,
        pub target: String,
        pub occurences: u32,
    }

    pub fn prepare_tests(name: String, in_directory: String, out_directory: String) -> Vec<Test> {
        let mut tests: Vec<Test> = Vec::new();
        for (quality, occurence) in OCCURENCES {
            tests.push(Test {
                name: name.to_string(),
                quality: quality.to_string(),
                origin: format!("{in_directory}/{FILENAME}_{quality}.jpg"),
                target: format!("{out_directory}/{FILENAME}_{name}_{quality}.jpg"),
                occurences: *occurence,
            });
        }
        tests
    }

    pub fn starting_test(test: &Test) {
        println!(
            "Running {} with quality {} and occurences {}",
            test.name, test.quality, test.occurences
        );
    }

    pub fn ending(test: &Test, duration: u128) {
        println!(
            "\nResult for {}-{}-{} ==> \t{} ms",
            test.name, test.quality, test.occurences, duration
        );
    }

    pub fn dot(c: u32) {
        if c % 50 == 0 {
            print!(".");
            io::stdout().flush().unwrap()
        }
    }
}

fn main() {
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    app.concurrency_set(1);

    let tests = global::prepare_tests(
        "thumbnailator".to_string(),
        "/home/hack/WLN/images".to_string(),
        "/tmp".to_string(),
    );

    tests
        .iter()
        .map(|test| {
            global::starting_test(test);
            let start = instant::Instant::now();

            for c in 0..test.occurences {
                let image = VipsImage::new_from_file(test.origin.as_str()).unwrap();
                global::dot(c);
                let thumbnail = ops::thumbnail_image(&image, global::WIDTH);

                if ops::jpegsave_with_opts(
                    &thumbnail.unwrap(),
                    test.target.as_str(),
                    &ops::JpegsaveOptions::default(),
                )
                .is_err()
                {
                    println!("error: {}", app.error_buffer().unwrap())
                }
            }
            global::ending(test, start.elapsed().as_millis());
        })
        .for_each(drop)
}
