use std::path::PathBuf;

mod com;

pub mod dialog_ffi;
use dialog_ffi::{IDialog, Result};

mod dialog_future;
use dialog_future::{multiple_return_future, single_return_future};

use super::utils::init_com;
use crate::backend::DialogFutureType;
use crate::{FileDialog, FileHandle};

//
// File Picker
//

use crate::backend::{AsyncFilePickerDialogImpl, FilePickerDialogImpl};

impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf> {
            init_com(|| {
                let dialog = IDialog::build_pick_file(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }
        run(self).ok()
    }

    fn pick_files(self) -> Option<Vec<PathBuf>> {
        fn run(opt: FileDialog) -> Result<Vec<PathBuf>> {
            init_com(|| {
                let dialog = IDialog::build_pick_files(&opt)?;
                dialog.show()?;
                dialog.get_results()
            })?
        }
        run(self).ok()
    }
}

impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret = single_return_future(move || IDialog::build_pick_file(&self));
        Box::pin(ret)
    }

    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let ret = multiple_return_future(move || IDialog::build_pick_files(&self));
        Box::pin(ret)
    }
}

//
// Folder Picker
//

use crate::backend::{AsyncFolderPickerDialogImpl, FolderPickerDialogImpl};

impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf> {
            init_com(|| {
                let dialog = IDialog::build_pick_folder(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }

        run(self).ok()
    }

    fn pick_folders(self) -> Option<Vec<PathBuf>> {
        fn run(opt: FileDialog) -> Result<Vec<PathBuf>> {
            init_com(|| {
                let dialog = IDialog::build_pick_folders(&opt)?;
                dialog.show()?;
                dialog.get_results()
            })?
        }
        run(self).ok()
    }
}

impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret = single_return_future(move || IDialog::build_pick_folder(&self));
        Box::pin(ret)
    }

    fn pick_folders_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        let ret = multiple_return_future(move || IDialog::build_pick_folders(&self));
        Box::pin(ret)
    }
}

//
// File Save
//

use crate::backend::{AsyncFileSaveDialogImpl, FileSaveDialogImpl};

impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        fn run(opt: FileDialog) -> Result<PathBuf> {
            init_com(|| {
                let dialog = IDialog::build_save_file(&opt)?;
                dialog.show()?;
                dialog.get_result()
            })?
        }

        run(self).ok()
    }
}

impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let ret = single_return_future(move || IDialog::build_save_file(&self));
        Box::pin(ret)
    }
}
