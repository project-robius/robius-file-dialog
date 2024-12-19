use std::path::{Path, PathBuf};

mod error;
mod sys;

pub use error::{Error, Result};

pub struct FileDialog {
    pub(crate) inner: sys::FileDialog,
}

impl FileDialog {
    pub fn new() -> Result<Self> {
        Ok(Self {
            inner: sys::FileDialog::new()?,
        })
    }

    pub fn title(mut self, title: &str) -> Self {
        self.inner.set_title(title);
        self
    }

    pub fn modal(mut self, is_modal: bool) -> Self {
        self.inner.set_modal(is_modal);
        self
    }

    pub fn accept_label(mut self, accept_label: &str) -> Self {
        self.inner.set_accept_label(accept_label);
        self
    }

    pub fn filters<'a, I>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = &'a FileFilter>,
    {
        self.inner.set_filters(filters);
        self
    }

    pub fn initial_file<P>(mut self, initial_file: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.inner.set_initial_file(initial_file);
        self
    }

    pub fn initial_dir<P>(mut self, initial_dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.inner.set_initial_dir(initial_dir);
        self
    }

    pub fn open_with<F>(mut self, callback: F)
    where
        F: FnOnce(Result<PathBuf>) + 'static,
    {
        self.inner.open_with(callback);
    }

    pub fn open_multiple_with<F>(mut self, callback: F)
    where
        F: FnOnce(Result<Vec<PathBuf>>) + 'static,
    {
        self.inner.open_multiple_with(callback);
    }

    pub fn save_with<F>(mut self, callback: F)
    where
        F: FnOnce(Result<PathBuf>) + 'static,
    {
        self.inner.save_with(callback);
    }
}

pub(crate) trait FileDialogImpl: Sized {
    fn new() -> Result<Self>;

    fn set_title(&mut self, title: &str);

    fn set_modal(&mut self, is_modal: bool);

    fn set_accept_label(&mut self, accept_label: &str);

    fn set_filters<'a, I>(&mut self, filters: I)
    where
        I: IntoIterator<Item = &'a FileFilter>;

    fn set_initial_file<P>(&mut self, initial_file: P)
    where
        P: AsRef<Path>;

    fn set_initial_dir<P>(&mut self, initial_dir: P)
    where
        P: AsRef<Path>;

    fn open_with<F>(&mut self, callback: F)
    where
        F: FnOnce(Result<PathBuf>) + 'static;

    fn open_multiple_with<F>(&mut self, callback: F)
    where
        F: FnOnce(Result<Vec<PathBuf>>) + 'static;

    fn save_with<F>(&mut self, callback: F)
    where
        F: FnOnce(Result<PathBuf>) + 'static;
}

pub struct FileFilter {
    pub(crate) inner: sys::FileFilter,
}

impl FileFilter {
    pub fn new() -> Self {
        Self {
            inner: sys::FileFilter::new(),
        }
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.inner.set_name(name);
        self
    }

    pub fn mime_types<'a, I>(&mut self, mime_types: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.inner.set_mime_types(mime_types);
        self
    }

    pub fn patterns<'a, I>(&mut self, patterns: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.inner.set_patterns(patterns);
        self
    }

    pub fn extensions<'a, I>(&mut self, extensions: I) -> &mut Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        self.inner.set_extensions(extensions);
        self
    }
}

pub(crate) trait FileFilterImpl {
    fn new() -> Self;

    fn set_name(&mut self, name: &str);

    fn set_mime_types<'a, I>(&mut self, mime_types: I)
    where
        I: IntoIterator<Item = &'a str>;

    fn set_patterns<'a, I>(&mut self, patterns: I)
    where
        I: IntoIterator<Item = &'a str>;

    fn set_extensions<'a, I>(&mut self, extensions: I)
    where
        I: IntoIterator<Item = &'a str>;
}
