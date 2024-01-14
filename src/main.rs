fn good_three_vowels(word: &str) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

#[allow(clippy::ptr_arg)]
fn bad_three_vowels(word: &String) -> bool {
    good_three_vowels(word)
}

fn main() {
    let ferris = "Ferris";
    let curious = "Curious";
    println!("{ferris}: {}", bad_three_vowels(&ferris.to_string()));
    println!("{curious}: {}", bad_three_vowels(&curious.to_owned()));

    // Would fail
    //println!("{ferris}: {}", bad_three_vowels(ferris));
    //println!("{curious}: {}", bad_three_vowels(curious));

    println!("{ferris}: {}", good_three_vowels(ferris));
    println!("{curious}: {}", good_three_vowels(curious));

    let sentence = "Once upon a time, there was a friendly curious crab named Ferris".to_string();
    for word in sentence.split_whitespace() {
        if good_three_vowels(word) {
            println!("{word} has three consecutive vowels!");
        }
    }
}
