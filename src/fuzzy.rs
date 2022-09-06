use ngrammatic::{CorpusBuilder, Pad, SearchResult};

pub fn search(s: &str, choices: Vec<&str>, threshold: f32) -> Vec<SearchResult> {
    let mut corpus = CorpusBuilder::new().arity(2).pad_full(Pad::Auto).finish();
    for item in choices {
        corpus.add_text(item);
    }
    corpus.search(s, threshold)
}
