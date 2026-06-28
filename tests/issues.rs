use std::time::Duration;

use parse_wiki_text_2::{Configuration, ParseError};

const MAX_EXEC: Duration = Duration::from_millis(200);

#[test]
fn issue_1() {
    let s = "{".repeat(40);
    let r = Configuration::default().parse_with_timeout(&s, MAX_EXEC);

    // todo fix this so we don't get an error
    match r {
        Err(ParseError::TimedOut {
            execution_time,
            output,
        }) => {
            let dif = execution_time - MAX_EXEC;
            assert!(
                dif < Duration::from_millis(10),
                "expected timeout to be within 10ms of MAX_EXEC, got {:?}",
                dif
            );

            assert!(
                !output.warnings.is_empty(),
                "expected warnings to be present"
            )
        }
        _ => panic!("expected timeout"),
    }
}

// A timeout firing while the plain-text arm walked the bytes of a multi-byte
// character flushed a text node on a non-char-boundary `scan_position` and
// panicked ("byte index N is not a char boundary"). The timeout flush now snaps
// back to a char boundary first. Repro: a long run of multi-byte characters with
// a near-zero timeout, so the timeout fires (at loop_counter == 10_000) with
// `scan_position` mid-character.
#[test]
fn timeout_flush_on_multibyte_char_does_not_panic() {
    let s = "€".repeat(20000); // 60000 bytes ≫ the 10_000-iteration check interval
    // 1µs stays above any plausible Instant resolution while the 60 KB input still
    // trips the timeout many checks before EOF (so the test isn't clock-flaky).
    let r = Configuration::default().parse_with_timeout(&s, Duration::from_micros(1));
    match r {
        Err(ParseError::TimedOut { .. }) => {} // graceful, no panic
        _ => panic!("expected timeout"),
    }
}
