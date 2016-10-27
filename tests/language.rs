extern crate tic_tac_toe;
use tic_tac_toe::language::Language;

#[test]
fn test_english() {
    let test_text = Language::english();

    assert_eq!("TIC TAC TOE", *test_text.title());
    assert_eq!("Winner", *test_text.winner());
    assert_eq!("Loser", *test_text.loser());
    assert_eq!("Tied", *test_text.tied());
}

#[test]
fn test_spanish() {
    let test_text = Language::spanish();

    assert_eq!("TIC TAC TOE", *test_text.title());
    assert_eq!("Ganador", *test_text.winner());
    assert_eq!("Perdedor", *test_text.loser());
    assert_eq!("Atado", *test_text.tied());
}

