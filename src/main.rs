/*
Meghin Cypher
- An idea that my girlfriend had while "doing something with the cat" around bedtime.
The basis of the idea is that the index of a character in the alphabet is run through a cypher, where
combinations of letters with a bindings table translates to the alphabet index of the given character.

License? Do whatever dude / dudette. I can't believe you're reading this right now.
*/

/// Given letter_bindings (of the shape [('c', 4)...]), and an unsanitized word, this function will
/// return a Vec<String> of the translated word after the Meghin cypher.
pub fn generate_chunk(letter_bindings: &Vec<(char, usize)>, unclean_word: String) -> Vec<String> {

    // Should be in the Rust standard library imo
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // Each character becomes a list of characters. word_groups stores these lists for each character
    let mut word_groups: Vec<String> = vec![];

    // Sanitizees the word by making everything upper-case
    // (which, by the way - did you know that it's faster to do a comparison against upper-case chars?)
    let word = unclean_word.to_ascii_uppercase();

    // For each letter...
    for c in word.chars() {
        let mut resulting_word: String = "".to_string();

        // 1. Get the index
        //          Number value (e.g 8=h) describing where in the alphabet it is
        let letter_index = alphabet.find(c).unwrap() + 1;

        // 2. Keep a bucket with how much of this character we've built
        let mut fulfillment_bucket: usize = 0;

        // 3. Go through the number bindings and pull the largest possible map
        loop {
            for (character, index) in letter_bindings {
                if fulfillment_bucket + index <= letter_index {
                    fulfillment_bucket += index;
                    resulting_word.push(character.clone());
                    break;
                }
            }
            if fulfillment_bucket >= letter_index {
                break;
            }
        }

        // 4. Push the list that represents this character
        word_groups.push(resulting_word.clone());
    }

    return word_groups.clone();
}

fn main() {
    println!("Hello, world!");

    // Set up your letter bindings (can be in a JSON file)
    let mut letter_bindings = vec![
        ('A', 1),
        ('S', 2),
        ('D', 3),
        ('F', 4),
        ('J', 7),
        ('K', 8),
        ('L', 9),
        ('O', 10)
    ];

    // Reversing the bindings means that as  they loop, the biggest one will be first (no sorting needed)
    letter_bindings.reverse();

    // Can obviously be put in a file
    let sentence = "Whatever sentence you want to write should go here containing no punctuation because lazy".split(' ');

    // Each character turns into a word. So, a word is a list of words. A sentence is a list of a list of words.
    let mut all_results: Vec<Vec<String>> = vec![];

    // Just go through and convert each word
    for word in sentence {
        all_results.push(generate_chunk(&letter_bindings, word.to_string()));
    }

    // We're lazy so we're just going to debug print the result
    println!("{:?}", all_results);
}
