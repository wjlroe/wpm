use std::time::Duration;

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct TypingResult {
    correct_words: i32,
    incorrect_words: i32,
    backspaces: i32,
    wpm: i32,
}

pub fn calc_wpm(
    wordlist: &Vec<String>,
    input: &Vec<&str>,
    duration: Duration,
    num_backspaces: i32,
) -> TypingResult {
    let mut typing_result = TypingResult::default();
    typing_result.backspaces = num_backspaces;
    for (entered_word, required_word) in input.iter().zip(wordlist.iter()) {
        if entered_word == required_word {
            typing_result.correct_words += 1;
        } else {
            typing_result.incorrect_words += 1;
        }
    }
    typing_result.wpm =
        (typing_result.correct_words as f64 / (duration.as_secs() as f64 / 60.0)).floor() as i32;
    typing_result
}

#[test]
fn test_calc_wpm() {
    let wordlist = "also|sentence|stop|she|men|see|been|from|we|follow|but|mother|too|form|this|went|to|then|show|have|only|now|around|help|family|old|write|grow|also|over|together|city|end|quite|with|might|eat|four|where|hard|their|take|year|see|place|leave|too|too|is|other|near|around|saw|did|into|question|work|between|your|face|without|tree|as|girl|if|enough|stop|still|put|on|side|there|hear|large|more|be|there|took|some|into|off|down|so|is|tell|way|large|thing|earth|move|their|much|list|small|family|know|under|try|mean|above|end|was|what|night|them|most|good|example|left|mile|that|why|give|because|sea|above|boy|has|go|book|later|eat|land|about|line|life|said|often|really|the|at|without|large|should|away|end|no|oil|any|while|being|before|away|from|light|found|other|open|below|sound|began|come|night|year|world|start|that|it|after|and|show|every|find|old|while|school|your|point|often|example|children|up|found|then|quickly|some|still|again|our|world|may|group|help|point|own|around|make|than|look|girl|sometimes|hand|idea|change|people|get|page|the|own|it's|land|play|last|kind|eye|once|write|you|are|young|take|found|up|once|white|thought|answer|next|still|hand|state|air|food|don't|story|say|of|they|through|keep|far|should|different|eye|been|such|few|through|close|before|below|question|word|and|mother|along|number|miss|sound|her|boy|soon|car|seem|make|food|left|call|where|after|did|answer|write|there|got|mile|line|number|feet|America|earth|it's|find|get|me|home|cut|say|again|home|play|light|give|my|most|will|went|turn|sound|name|could|let|almost|head|carry|look|work|turn|letter|come|new|spell|mountain|move|children|air|live|this|hear|or|every|these|song|can|move|watch|which|picture|own|was|right|does|need|important|river|some|had|after|or|man|study|should|part|would|and|by|watch|earth|head";
    let words = wordlist.split('|').collect::<Vec<_>>();
    let input = "also sentence stop she men see been from we follow but mother too form this went to then show have only now around help family old write grow also over together city end quite4 with might eat four where hard their take year see place learve too too is other near around saw did into question work between your face without tree as girl if enough stop still put on side there hear large more be there took some into off down so is";
    let input_words = input.split(' ').collect::<Vec<_>>();
    let start_time = SystemTime::UNIX_EPOCH.add(Duration::from_secs(1554148227));
    let end_time = SystemTime::UNIX_EPOCH.add(Duration::from_secs(1554148287));
    let duration = end_time.duration_since(start_time).unwrap();
    let num_backspaces = 11;
    assert_eq!(
        82,
        calc_wpm(&words, &input_words, duration, num_backspaces).wpm
    );

    assert_eq!(
        164,
        calc_wpm(
            &words,
            &input_words,
            Duration::from_secs(30),
            num_backspaces
        )
        .wpm
    );
}
