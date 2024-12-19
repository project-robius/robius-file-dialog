use std::path::{Path, PathBuf};

use gtk4::gio;

use crate::{FileDialogImpl, FileFilterImpl};

pub(crate) struct FileDialog {
    file_dialog: gtk4::FileDialog,
}

impl FileDialogImpl for FileDialog {
    fn new() -> crate::Result<Self> {
        gtk4::init().unwrap(); // XXX
        Ok(Self {
            file_dialog: gtk4::FileDialog::new(),
        })
    }

    fn set_title(&mut self, title: &str) {
        self.file_dialog.set_title(title);
    }

    fn set_modal(&mut self, is_modal: bool) {
        self.file_dialog.set_modal(is_modal);
    }

    fn set_accept_label(&mut self, accept_label: &str) {
        self.file_dialog.set_accept_label(Some(accept_label));
    }

    fn set_filters<'a, I>(&mut self, filters: I)
    where
        I: IntoIterator<Item = &'a crate::FileFilter>,
    {
        // TODO ListModel conversion
        todo!()
    }

    fn set_initial_file<P>(&mut self, initial_file: P)
    where
        P: AsRef<Path>,
    {
        self.file_dialog
            .set_initial_file(Some(&gio::File::for_path(initial_file)));
    }

    fn set_initial_dir<P>(&mut self, initial_dir: P)
    where
        P: AsRef<Path>,
    {
        self.file_dialog
            .set_initial_folder(Some(&gio::File::for_path(initial_dir)));
    }

    fn open_with<F>(&mut self, callback: F)
    where
        F: FnOnce(crate::Result<PathBuf>) + 'static,
    {
        todo!()
    }

    fn open_multiple_with<F>(&mut self, callback: F)
    where
        F: FnOnce(crate::Result<Vec<PathBuf>>) + 'static,
    {
        todo!()
    }

    fn save_with<F>(&mut self, callback: F)
    where
        F: FnOnce(crate::Result<PathBuf>) + 'static,
    {
        todo!()
    }
}

pub struct FileFilter {
    file_filter: gtk4::FileFilter,
}

impl FileFilterImpl for FileFilter {
    fn new() -> Self {
        Self {
            file_filter: gtk4::FileFilter::new(),
        }
    }

    fn set_name(&mut self, name: &str) {
        self.file_filter.set_name(Some(name));
    }

    fn set_mime_types<'a, I>(&mut self, mime_types: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        for mime_type in mime_types.into_iter() {
            self.file_filter.add_mime_type(mime_type);
        }
    }

    fn set_patterns<'a, I>(&mut self, patterns: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        for pattern in patterns.into_iter() {
            self.file_filter.add_pattern(pattern);
        }
    }

    fn set_extensions<'a, I>(&mut self, extensions: I)
    where
        I: IntoIterator<Item = &'a str>,
    {
        for extension in extensions.into_iter() {
            self.file_filter.add_suffix(extension);
        }
    }
}
