use proc_macro2::TokenStream;
pub use quote::quote;
use std::path::{Path, PathBuf};
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

#[derive(Default)]
pub struct Story {
    pub name: String,
    pub body: TokenStream,
    pub dependent_files: Vec<PathBuf>,
    pub asset_files: Vec<PathBuf>,
}

impl Story {
    pub fn new(name: impl ToString, body: TokenStream) -> Self {
        Self {
            name: name.to_string(),
            body,
            ..Default::default()
        }
    }

    pub fn add_dependent_file(mut self, path: PathBuf) -> Self {
        self.dependent_files.push(path);
        self
    }

    pub fn add_asset_file(mut self, path: PathBuf) -> Self {
        self.asset_files.push(path);
        self
    }
}

#[macro_export]
macro_rules! story_body {
    ($($tt:tt)*) => {
        $crate::story_book::quote! { $($tt)* }
    };
}
