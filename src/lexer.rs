/// State design pattern implementation of a finite automaton.
/// Right now it only recognizes the set of strings representing real numbers.

struct Lexer {
}

impl Lexer {
    fn accepts(input: &str) -> bool {
        let mut state: Box<dyn State> = Box::new(Start {});
        for c in input.chars() {
            state = state.consume(c);
        }
        
        state.accepting()
    }
}

trait State {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State>;
    fn accepting(&self) -> bool;
}

struct Start {}

impl State for Start {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '+' | '-' => Box::new(Sign {}),
            '0'..='9' => Box::new(WholePart {}),
            _ => Box::new(Error {}),
        }
    }

    fn accepting(&self) -> bool {
        false
    }
}

struct Sign {}

impl State for Sign {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(WholePart {}),
            _ => Box::new(Error {}),
        }
    }
    
    fn accepting(&self) -> bool {
        false
    }
}

struct Error {}

impl State for Error {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        self
    }

    fn accepting(&self) -> bool {
        false
    }
}

struct WholePart {}

impl State for WholePart {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            '.' => Box::new(DecimalPoint {}),
            'e' | 'E' => Box::new(E {}),
            _ => Box::new(Error {}),
        }
    }

    fn accepting(&self) -> bool {
        true
    }
}

struct DecimalPoint {}

impl State for DecimalPoint {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(FractionalPart {}),
            _ => Box::new(Error {}),
        }
    }
    
    fn accepting(&self) -> bool {
        false
    }
}

struct FractionalPart {}

impl State for FractionalPart {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            'e' | 'E' => Box::new(E {}),
            _ => Box::new(Error {}),
        }
    }

    fn accepting(&self) -> bool {
        true
    }
}

struct E {}

impl State for E {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => Box::new(ScientificPart {}),
            _ => Box::new(Error {}),
        }

    }
        
    fn accepting(&self) -> bool {
        false
    }
}

struct ScientificPart {}

impl State for ScientificPart {
    fn consume(self: Box<Self>, c: char) -> Box<dyn State> {
        match c {
            '0'..='9' => self,
            _ => Box::new(Error {}),
        }
    }

    fn accepting(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn assert_accepts(input: &str) {
        assert!(Lexer::accepts(input));
    }

    fn assert_rejects(input: &str) {
        assert!(!Lexer::accepts(input));
    }

    #[test]
    fn lexer_accepts_floating_point_numbers() {
        assert_accepts("+1");
        assert_accepts("1392480");
        assert_accepts("3.14");
        assert_accepts("-1");
        assert_accepts("-2230498.0940258340985");
    }

    #[test]
    fn lexer_accepts_scientific_notation() {
        assert_accepts("1E10");
        assert_accepts("3.14e2");
    }

    #[test]
    fn lexer_rejects_misplaced_e() {
        assert_rejects("e");
        assert_rejects("1e");
        assert_rejects("e1");
    }

    #[test]
    fn lexer_rejects_misplaced_sign() {
        assert_rejects("-");
        assert_rejects("1-");
        assert_rejects("+");
        assert_rejects("1+3");
        assert_rejects("1e+3");
    }

    #[test]
    fn lexer_rejects_misplaced_decimal_point() {
        assert_rejects(".");
        assert_rejects("1.");
        assert_rejects(".1");
        assert_rejects("0.0.1");
        assert_rejects("3.e4");
        assert_rejects("3e.1");
    }

    #[test]
    fn lexer_rejects_invalid_characters() {
        assert_rejects("a");
        assert_rejects("3.a4");
    }
}
