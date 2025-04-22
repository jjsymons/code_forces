// 4A. Watermelon
// time limit per test1 second
// memory limit per test64 megabytes

// One hot summer day Pete and his friend Billy decided to buy a watermelon. They chose the biggest and the ripest one,
// in their opinion. After that the watermelon was weighed, and the scales showed w kilos. They rushed home, dying of
// thirst, and decided to divide the berry, however they faced a hard problem.

// Pete and Billy are great fans of even numbers, that's why they want to divide the watermelon in such a way that each
// of the two parts weighs even number of kilos, at the same time it is not obligatory that the parts are equal. The
// boys are extremely tired and want to start their meal as soon as possible, that's why you should help them and find
// out, if they can divide the watermelon in the way they want. For sure, each of them should get a part of positive weight.

// Input
// The first (and the only) input line contains integer number w (1 ≤ w ≤ 100) — the weight of the watermelon bought by
// the boys.

// Output
// Print YES, if the boys can divide the watermelon into two parts, each of them weighing even number of kilos; and NO in
// the opposite case.

// Examples:

//     Input
//     8
//     Output
//     YES

// Note

// For example, the boys can divide the watermelon into two parts of 2 and 6 kilos respectively (another variant — two
// parts of 4 and 4 kilos)."


pub fn run() {
    println!("Challenge 4_a:");
    println!("When the boys split four water melons weighing 4, 33, 7 and 100 kilos, they expect them to split Yes, No, No, and Yes respectively");
    println!("Melon weighing 4 kilos: {}", odd_or_even(4));
    println!("Melon weighing 33 kilos: {}", odd_or_even(33));
    println!("Melon weighing 7 kilos: {}", odd_or_even(7));
    println!("Melon weighing 100 kilos: {}", odd_or_even(100));
}

fn odd_or_even(weight: u32) -> String {
    if weight % 2 == 0 {
        "Yes".to_string()
    } else {
        "No".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercise_4a_odd_or_even() {
        assert_eq!(odd_or_even(3), "No");
        assert_eq!(odd_or_even(6), "Yes");
        assert_eq!(odd_or_even(80), "Yes");
    }
}

