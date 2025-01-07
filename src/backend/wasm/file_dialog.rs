use std::future::ready;

use crate::backend::DialogFutureType;
use crate::file_dialog::FileDialog;
use crate::FileHandle;

//
// File Save
//

use crate::backend::AsyncFileSaveDialogImpl;

impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        let file = FileHandle::writable(self);
        Box::pin(ready(Some(file)))
    }
}
