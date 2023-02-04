pub fn map<F, I, O>(input: Vec<I>, mut function: F) -> Vec<O>
where
    F: FnMut(I) -> O,
{
    let mut result = Vec::new();
    for v in input {
        result.push(function(v));
    }
    result
}
