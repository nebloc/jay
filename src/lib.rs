use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn jay(numberplate: &str) -> i32 {
    let mut score = 0;
    let mut wildcard = false;

    for c in numberplate.to_uppercase().chars() {
        if c >= 'A' && c <= 'Z' {
            score += c as i32 - 64
        } else if c > '0' && c <= '9' {
            score += c as i32 - 48
        } else if c == '0' {
            wildcard = true;
        }
    }

    if wildcard == true {
        score += 7 - (score % 7)
    }

    return score;
}

#[wasm_bindgen]
pub fn liked(score: i32) -> bool {
    if score < 1 {
        return false
    }
    return score % 7 == 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn correctly_scores_num() {
        assert_eq!(jay("LC58 UYR"), 92);
    }
    #[test]
    fn correctly_scores_lowercase() {
        assert_eq!(jay("lc58 uyr"), 92);
    }
    #[test]
    fn correctly_scores_mixedcase() {
        assert_eq!(jay("lC58 uYr"), 92);
    }
    #[test]
    fn correctly_scores_special_char() {
        assert_eq!(jay("lC58&*uY@^&!r"), 92);
    }
    #[test]
    fn handles_empty_string() {
        assert_eq!(jay(""), 0); // TODO: Is this the expected behaviour we actually want?
    }
    #[test]
    fn correctly_returns_liked_plate() {
        let score = jay("LI58 UYR");
        assert_eq!(score, 98); // Liked plate has correct score and is liked
        assert_eq!(score % 7 == 0, true); // Score is divisible by 7, hence liked
    }
    #[test]
    fn handles_short_plate() {
        assert_eq!(jay("A6"), 7);
    }
    #[test]
    fn handles_long_plate() {
        assert_eq!(jay("KA11MC9499"), 61);
    }
    #[test]
    fn handles_zero_wildcard() {
        assert_eq!(jay("LC08 UYR"), 91);
    }
    #[test]
    fn handles_zeroscore() {
        assert_eq!(liked(0), false);
        assert_eq!(liked(-1), false);
    }
    #[test]
    fn liked_works() {
        assert_eq!(liked(6), false);
        assert_eq!(liked(7), true);
        assert_eq!(liked(63), true);
        assert_eq!(liked(64), false);
    }
}
