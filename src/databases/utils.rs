// Since sqlx does not support multiple databases in one crate
// some utils try to replicate some of marcos' functionality

const VALUE_PLACEHOLDER: char = '?';

pub fn query_insert(mut query: String, values: Vec<&str>) -> Option<String>{
    for value in values.iter() {
        let index = query.find(VALUE_PLACEHOLDER)?;
        query.remove(index);
        query.insert_str(index, &value);
    };
    Some(query)
}

#[cfg(test)]
mod tests {
    use super::query_insert;

    #[test]
    fn query_insert_test1() {
        let query = "foo?".to_string();
        assert_eq!("foobar".to_string(), query_insert(query, vec!["bar"]).unwrap())
    }

    #[test]
    fn query_insert_test2() {
        let query = "foo?bar?".to_string();
        let parsed_query = query_insert(query, vec!["bar", "foo"]).unwrap();
        assert_eq!("foobarbarfoo".to_string(), parsed_query)
    }
}