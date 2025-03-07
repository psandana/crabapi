use std::io;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum FileOpenDialogError {
    DialogClosed,
    IoError(io::ErrorKind),
}

pub async fn open_file() -> Result<(PathBuf, Arc<String>), FileOpenDialogError> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a file...")
        .pick_file()
        .await
        .ok_or(FileOpenDialogError::DialogClosed)?;

    load_file(picked_file).await
}

pub async fn load_file(
    path: impl Into<PathBuf>,
) -> Result<(PathBuf, Arc<String>), FileOpenDialogError> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| FileOpenDialogError::IoError(error.kind()))?;

    Ok((path, contents))
}
