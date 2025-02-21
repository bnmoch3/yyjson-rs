use yyjson_sys::add;

pub fn add2(a: u64, b: u64) -> u64 {
    add(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
