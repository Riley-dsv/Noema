use crate::cli::SearchArgs;

pub struct SearchOptions<'a> {
    pub keyword: Option<&'a str>,
    pub tags: Option<Vec<&'a str>>,
    pub match_any: bool,
    pub search_title: bool,
    pub search_content: bool,
    pub limit: Option<usize>,
}

impl<'a> From<&'a SearchArgs> for SearchOptions<'a> {
    fn from(value: &'a SearchArgs) -> Self {
        Self {
            keyword: value.keyword.as_deref(),
            tags: value
                .tags
                .as_ref()
                .map(|tags| tags.iter().map(String::as_str).collect()),
            match_any: value.match_any,
            search_title: value.title,
            search_content: value.content,
            limit: value.limit,
        }
    }
}
