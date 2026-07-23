//! Control-plane command parser corpus (`--features harness` only).

use ronin::harness_control::{parse_harness_command, HarnessControlCommand};

#[derive(Clone, Copy)]
enum Expected {
    Ping,
    Open(&'static str),
    Scroll(i32),
    None,
}

struct ParseCase {
    line: &'static str,
    expect: Expected,
}

const CASES: &[ParseCase] = &[
    ParseCase {
        line: "ping",
        expect: Expected::Ping,
    },
    ParseCase {
        line: "  ping  ",
        expect: Expected::Ping,
    },
    ParseCase {
        line: "open thr_0",
        expect: Expected::Open("thr_0"),
    },
    ParseCase {
        line: "open thr_42",
        expect: Expected::Open("thr_42"),
    },
    ParseCase {
        line: "open  spaced_id  ",
        expect: Expected::Open("spaced_id"),
    },
    ParseCase {
        line: "scroll 0",
        expect: Expected::Scroll(0),
    },
    ParseCase {
        line: "scroll -40",
        expect: Expected::Scroll(-40),
    },
    ParseCase {
        line: "scroll 120",
        expect: Expected::Scroll(120),
    },
    ParseCase {
        line: "scroll not_a_number",
        expect: Expected::None,
    },
    ParseCase {
        line: "open ",
        expect: Expected::None,
    },
    ParseCase {
        line: "open",
        expect: Expected::None,
    },
    ParseCase {
        line: "scroll",
        expect: Expected::None,
    },
    ParseCase {
        line: "nope",
        expect: Expected::None,
    },
    ParseCase {
        line: "",
        expect: Expected::None,
    },
    ParseCase {
        line: "PING",
        expect: Expected::None,
    },
];

#[test]
fn harness_control_parse_corpus() {
    for c in CASES {
        let got = parse_harness_command(c.line);
        match c.expect {
            Expected::Ping => assert_eq!(got, Some(HarnessControlCommand::Ping)),
            Expected::Open(id) => {
                assert_eq!(
                    got,
                    Some(HarnessControlCommand::OpenThread {
                        thread_id: id.into()
                    })
                );
            }
            Expected::Scroll(delta) => {
                assert_eq!(got, Some(HarnessControlCommand::ScrollMessages { delta }));
            }
            Expected::None => assert_eq!(got, None),
        }
    }
}

#[test]
fn harness_control_open_thread_ids_matrix() {
    for i in 0..80 {
        let id = format!("thread_{i}");
        let line = format!("open {id}");
        let got = parse_harness_command(&line).expect("parse");
        assert_eq!(
            got,
            HarnessControlCommand::OpenThread {
                thread_id: id.clone()
            }
        );
    }
}

#[test]
fn harness_control_scroll_delta_matrix() {
    for delta in -50..=50 {
        let line = format!("scroll {delta}");
        let got = parse_harness_command(&line).expect("parse scroll");
        assert_eq!(got, HarnessControlCommand::ScrollMessages { delta });
    }
}
