use std::time::{Duration, Instant};
use wpm::TypingTest;

#[test]
fn test_some_words() {
    let mut typing_test = TypingTest::default();
    let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
    let words = wordlist
        .split('|')
        .map(|word| word.to_string())
        .collect::<Vec<_>>();
    let end_time = Instant::now();
    let duration = Duration::from_secs(60);
    let start_time = end_time - duration;
    typing_test.set_words(words);
    typing_test.duration = Some(duration);
    // fake start time because it uses elapsed time internally
    typing_test.start_time = Some(Instant::now());
    typing_test.backspaces = 11;
    assert_eq!(Some(false), typing_test.is_done(), "Test is not yet done");
    assert_eq!(false, typing_test.ended, "Test not ended");

    let input = "also sentence stop she men see been from we follow but mother too form this went to then show have only now around help family old write grow also over together city end quite4 with might eat four where hard their take year see place learve too too is other near around saw did into question work between your face without tree as girl if enough stop still put on side there hear large more be there took some into off down so is";
    for character in input.chars() {
        typing_test.typed_char(character);
    }
    typing_test.start_time = Some(start_time);
    typing_test.end(); // Hoover up the final word
    typing_test.ended = true;

    assert_eq!(Some(true), typing_test.is_done(), "Test is done");

    let typing_result = typing_test.result();
    assert_eq!(11, typing_result.backspaces, "11 backspaces");
    assert_eq!(82, typing_result.correct_words, "82 correct words");
    assert_eq!(82, typing_result.wpm, "82 WPM");
}

#[test]
fn test_correct_so_far() {
    let mut typing_test = TypingTest::default();
    typing_test.words = vec![String::from("factotum"), String::from("blah")];
    let first_word = typing_test.words.get(0).unwrap();
    dbg!(first_word);
    assert!(
        typing_test.correct_so_far(),
        "Nothing typed, so correct so far"
    );
    typing_test.typed_char('f');
    assert!(
        typing_test.correct_so_far(),
        "One character typed, correct so far..."
    );
    typing_test.typed_char('a');
    assert!(
        typing_test.correct_so_far(),
        "Two characters typed, correct so far..."
    );
    typing_test.typed_char('z');
    assert!(
        !typing_test.correct_so_far(),
        "Now an incorrect char has been entered..."
    );
    typing_test.backspace();
    assert!(
        typing_test.correct_so_far(),
        "Two characters typed, correct so far..."
    );
    typing_test.typed_char('c');
    assert!(
        typing_test.correct_so_far(),
        "More characters typed, correct so far..."
    );
    typing_test.typed_char('t');
    assert!(
        typing_test.correct_so_far(),
        "More characters typed, correct so far..."
    );
    typing_test.typed_char('o');
    assert!(
        typing_test.correct_so_far(),
        "More characters typed, correct so far..."
    );
    typing_test.typed_char('t');
    assert!(
        typing_test.correct_so_far(),
        "More characters typed, correct so far..."
    );
    typing_test.typed_char('u');
    assert!(
        typing_test.correct_so_far(),
        "More characters typed, correct so far..."
    );
    typing_test.typed_char('m');
    assert!(
        typing_test.correct_so_far(),
        "All characters typed, correct so far..."
    );
    typing_test.typed_char('z');
    assert!(
        !typing_test.correct_so_far(),
        "Now an incorrect char has been entered..."
    );
    typing_test.backspace();
    assert!(
        typing_test.correct_so_far(),
        "All characters typed, correct so far..."
    );
}
