use std::ops::Rem;

// This was a really tough one - I had to look up some hints from other
// submissions to get past the lifetime issues.

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T> {
    function: Box<dyn Fn(T) -> bool + 'a>,
    substitute: String,
}

impl<'a, T> Matcher<'a, T>
where
    T: ToString + Copy,
{
    pub fn new<F, S>(function: F, substitute: S) -> Self
    where
        F: Fn(T) -> bool + 'a,
        S: ToString,
    {
        Self {
            function: Box::new(function),
            substitute: substitute.to_string(),
        }
    }

    pub fn apply(&self, number: T) -> Option<&str> {
        if (self.function)(number) {
            Some(&self.substitute)
        } else {
            None
        }
    }
}

pub struct Fizzy<'a, T> {
    matchers: Vec<Matcher<'a, T>>,
}

impl<'a, T> Fizzy<'a, T>
where
    T: ToString + Copy + 'a,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    fn apply_matchers(&self, element: T) -> String {
        let result = self
            .matchers
            .iter()
            .fold(String::new(), |mut acc, matcher| {
                if let Some(substitute) = matcher.apply(element) {
                    acc.push_str(&substitute);
                }
                acc
            });

        if result.len() != 0 {
            result
        } else {
            element.to_string()
        }
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: Iterator<Item = T> + 'a,
    {
        iter.map(move |e| self.apply_matchers(e))
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T>() -> Fizzy<'a, T>
where
    T: From<u8> + Rem<Output = T> + ToString + Copy + Rem<Output = T> + PartialEq + 'a,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}
