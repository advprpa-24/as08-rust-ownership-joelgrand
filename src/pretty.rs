use crate::term::*;
use std::fmt;

/// Pretty prints a term.
pub fn pretty_print(term: &Term) -> String {
    //format!("{:?}", term)
    // TODO: Implement pretty printing for lambda calculus terms.
    match term {
        Term::Var(a) => format!("{a}"),
        Term::Abs(a, b) => format!{"λ{a} ({})", pretty_print(&*b)},
        Term::App(b1, b2) => format!{"{} {}", pretty_print(&*b1), pretty_print(&*b2)}
    }
}

/// Display trait implementation for Term.
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", pretty_print(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vars() {
        let x = var("x");
        let y = var("y");
        let z = var("z");

        assert_eq!(pretty_print(&x), String::from("x"));
        assert_eq!(pretty_print(&y), String::from("y"));
        assert_eq!(pretty_print(&z), String::from("z"));
    }

    #[test]
    fn test_abs() {
        let x1 = abs("x", var("y"));
        let x2 = abs("x", abs("y", var("z")));

        assert_eq!(pretty_print(&x1), String::from("λx (y)"));
        assert_eq!(pretty_print(&x2), String::from("λx (λy (z))"));

    }


    #[test]
    fn test_apps() {
        let input = app(abs("x", app(var("x"), var("y"))), var("z"));
        let output = String::from("λx (x y z)");

        assert_eq!(pretty_print(&input), output);
    }
}