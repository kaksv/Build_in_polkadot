pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn explore_it() {
        assert_eq!(2+2, 4);
    }
}
//Declaring we must put above the test #[cfg(test)]
