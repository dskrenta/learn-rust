extern crate regex;

use regex::Regex;

pub struct QueryParser {
    query: String,
    terms: Vec<String>,
    query_vector: Vec<u32>,
}

impl QueryParser {
    pub fn new(query: &str) -> QueryParser {
        QueryParser {
            query: String::from(query),
            terms: Vec::<String>::new(),
            query_vector: Vec::<u32>::new(),
        }
    }

    pub fn process_string(&mut self) {
        self.normalize_terms();
        self.tokenize_terms();
        self.remove_stop_words();
    }

    pub fn vectorize_query(&self) {

    }

    fn normalize_terms(&mut self) {
        self.query.to_lowercase();
        let re = Regex::new(r"[.,\/#!$%\^&\*;:{}=\-_`~()]").unwrap();
        re.replace_all(&self.query, "");
    }

    fn tokenize_terms(&mut self) {
        self.terms = self.query.split_whitespace().map(|term| term.to_string()).collect();
    }

    fn remove_stop_words(&mut self) {
        self.terms.retain(|term| term != "hello");
    }
}
