use lazy_static::lazy_static;
use regex::{self, Regex};

lazy_static! {
    static ref SEMVER: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
    static ref TEXT: &'static str = "\
        In the beginning, there was 1.0.0.\
        For a while, we used the 1.0.0-beta, \
        but in the end, we settled on 1.24.5.";
}

#[test]
fn is_match() {
    let example = r##"regex = "0.12.105""##;
    assert!(SEMVER.is_match(example));

    let example = r"sadsd0.12.105cascc";
    assert!(SEMVER.is_match(example));

    let example = r"sadsd0.12.105-alpha";
    assert!(SEMVER.is_match(example));

    let example = r"sadsd0.12.a105";
    assert!(!SEMVER.is_match(example));
}

#[test]
fn captures() {
    let example = r##"regex = "0.12.105-alpha-beta" 1.205.0-delta""##;
    let captures = SEMVER
        // captures only return capture groups for the the first match.
        .captures(example)
        // There is a least one match, so can unwrap()
        .unwrap();

    assert_eq!(&captures[0], "0.12.105-alpha-beta");
    assert_eq!(&captures[1], "0");
    assert_eq!(&captures[2], "12");
    assert_eq!(&captures[3], "105");
    assert_eq!(&captures[4], "-alpha-beta");

    // prints the capture groups that participated in the first match only.
    for (id, capture) in captures.iter().enumerate() {
        if let Some(t) = capture {
            println!("{}", t.as_str());
        } else {
            println!("Group: {} was not present!", id);
        }
    }
}

#[test]
fn captures_iter() {
    let example = r##"regex = "0.12.105-alpha-beta" 1.205.0-delta""##;
    let matches = vec!["0.12.105-alpha-beta", "1.205.0-delta"];

    // capture_iter contains all capture groups that matched in the text,
    // not only the first match in the text.
    let captures = SEMVER.captures_iter(example);
    for (id, capture) in captures.enumerate() {
        println!(
            "Capture group 0 for match number: {}\n\t{}",
            id, &capture[0]
        );
        assert_eq!(matches[id], &capture[0]);
    }
}

#[test]
// find_iter is less expensive than capture, so if you
// don't actually need the captures, prefer this call.
fn find_iter() {
    let matches = SEMVER
        .find_iter(*TEXT)
        .map(|m| m.as_str())
        .collect::<Vec<_>>();

    assert_eq!(matches, vec!["1.0.0", "1.0.0-beta", "1.24.5"]);
}

#[test]
// find will only return the first match in the text search,
// if no matches at all were found, returns None.
fn find() {
    let first_match = SEMVER.find(*TEXT).unwrap();

    assert_eq!(first_match.as_str(), "1.0.0");
}
