// A. Way Too Long Words

// Sometimes some words like "localization" or "internationalization" are so long that writing them many times
// in one text is quite tiresome.

// Let's consider a word too long, if its length is strictly more than 10 characters. All too long words should be 
// replaced with a special abbreviation.

// This abbreviation is made like this: we write down the first and the last letter of a word and between them we 
// write the number of letters between the first and the last letters. That number is in decimal system and doesn't 
// contain any leading zeroes.

// Thus, "localization" will be spelt as "l10n", and "internationalization» will be spelt as "i18n".

// You are suggested to automatize the process of changing the words with abbreviations. At that all too long words 
// should be replaced by the abbreviation and the words that are not too long should not undergo any changes.

// Input
// The first line contains an integer n (1 ≤ n ≤ 100). Each of the following n lines contains one word. All the words 
// consist of lowercase Latin letters and possess the lengths of from 1 to 100 characters.

// Output
// Print n lines. The i-th line should contain the result of replacing of the i-th word from the input data.

// Examples
// Input
// 4
// word
// localization
// internationalization
// pneumonoultramicroscopicsilicovolcanoconiosis

// Output
// word
// l10n
// i18n
// p43s

use std::ptr::null;

pub fn run(){
    println!("Question 71A: Way too long words! For the following words, 4, word, localization, internationalization,\n pneumonoultramicroscopicsilicovolcanoconiosis. Answers should be BLANK, word, l10n i18n, p43s respectively");
    println!("{}", way_to_long_words("4"));
    println!("{}", way_to_long_words("word"));
    println!("{}", way_to_long_words("localization"));
    println!("{}", way_to_long_words("November"));
    println!("{}", way_to_long_words("Utilitarianism"));
}

fn way_to_long_words(mut word: &str) -> String {
    word = word.trim(); 
    for chars in word.chars() {
        if chars.is_numeric(){
            return "".to_string();
        }
    }
    if word.len() >= 10 {
        let output = format!("{}{}{}", word.chars().nth(0).unwrap() , word.len() - 2, word.chars().nth(word.len() - 1).unwrap());
        output
    } else {
        word.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_71a_way_to_long_words() {
        assert_eq!(way_to_long_words("1990"), "");
        assert_eq!(way_to_long_words("h3llo"), "");
        assert_eq!(way_to_long_words("uncharacteristically "), "u18y"); 
        assert_eq!(way_to_long_words(" antidisestablishmentarianism"), "a26m");
        assert_eq!(way_to_long_words("esophagogastroduodenoscopy"), "e24y");
    }
}

