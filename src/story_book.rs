use proc_macro2::TokenStream;
pub use quote::quote;
use std::path::PathBuf;
use std::sync::Arc;

pub struct StoryBookBuilder {
    stories: Vec<Story>,
}

impl StoryBookBuilder {
    pub fn new() -> Self {
        Self { stories: vec![] }
    }

    pub fn add_story(mut self, story: Story) -> Self {
        self.stories.push(story);
        self
    }

    pub fn build(self) -> StoryBook {
        StoryBook {
            stories: self.stories.into_iter().map(Arc::new).collect(),
        }
    }
}

pub struct StoryBook {
    pub stories: Vec<Arc<Story>>,
}

pub struct Story {
    pub name: String,
    pub body: TokenStream,
    pub dependent_files: Vec<PathBuf>,
}

impl Story {
    pub fn new(name: impl ToString, body: TokenStream, dependent_files: Vec<PathBuf>) -> Self {
        Self {
            name: name.to_string(),
            body,
            dependent_files,
        }
    }
}

#[macro_export]
macro_rules! story_body {
    ($($tt:tt)*) => {
        $crate::story_book::quote! { $($tt)* }
    };
}
