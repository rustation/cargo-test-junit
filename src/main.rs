extern crate test_to_vec;
extern crate nom;
extern crate sxd_document;
extern crate clap;
extern crate duct;

use nom::IResult;
use sxd_document::Package;
use sxd_document::writer::format_document;
use test_to_vec::Suite;
use std::fs;

mod doc;
mod args;

fn main() {
    let ref matches = args::get_args();

    let sub_match = matches.subcommand_matches("test-junit")
        .unwrap();

    let features = sub_match.value_of("features")
        .map(|x| format!(" --features {}", x))
        .unwrap_or("".to_string());

    let ref name = args::get_file_name(matches).unwrap();

    let output = get_test_output(features)
        .map_err(|x| {
            if let duct::Error::Status(ref output) = x {
                println!("{}", String::from_utf8_lossy(&output.stdout))
            }

            x
        })
        .unwrap();

    let package = Package::new();
    let d = package.as_document();

    let suites: Vec<Suite> = match test_to_vec::cargo_test_result_parser(&output.stdout) {
        IResult::Done(_, x) => x,
        IResult::Error(_) => panic!("Parser error"),
        _ => panic!("Parser did not finish successfully"),
    };

    let (totals, failures) = suites.iter()
        .fold((0, 0),
              |(total, failed), y| (total + y.total, failed + y.failed));

    let test_suites = doc::el(d, "testsuites")
        .attr("name", name)
        .attr("errors", failures)
        .attr("tests", totals);

    doc::append_child(d, &test_suites);

    for suite in &suites {
        let test_suite = doc::el(d, "testsuite")
            .attr("name", suite.name)
            .attr("errors", suite.failed)
            .attr("failures", suite.failed)
            .attr("tests", suite.total)
            .append_to(&test_suites);

        for &test_to_vec::Test { name, error, .. } in &suite.tests {
            let test_case = doc::el(d, "testcase")
                .attr("name", name)
                .append_to(&test_suite);

            if let Some(e) = error {
                doc::el(d, "failure")
                    .attr("message", e)
                    .append_to(&test_case);
            }
        }
    }

    let mut f = fs::File::create(format!("{}.xml", name))
        .expect(&format!("could not create file: {}", name));

    format_document(&d, &mut f)
        .ok()
        .expect(&format!("unable to output XML to {}", name));
}

fn get_test_output(features: String) -> Result<duct::Output, duct::Error> {
    duct::sh(format!("cargo test{}", features))
        .stderr_to_stdout()
        .capture_stdout()
        .unchecked()
        .run()
}
