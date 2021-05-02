use std::path::{Path, PathBuf};

use crate::snapshot::{Snapshot, SnapshotRec};

pub struct Project {
    name: String,
    pub(crate) snapshots: Vec<Snapshot>,
}

impl Project {
    pub fn new<N: ToString>(name: N) -> Self {
        Self {
            name: name.to_string(),
            snapshots: Vec::new(),
        }
    }

    pub fn start_snapshot(self) -> SnapshotRec {
        SnapshotRec::new(self)
    }

    pub fn output_rec<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();

        if path.exists() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "a file with that name already exists",
            )));
        }

        let dir_path = path
            .parent()
            .map(|p| PathBuf::from(p))
            .unwrap_or(PathBuf::from("./"));

        std::fs::DirBuilder::new()
            .recursive(true)
            .create(dir_path)?;

        let serialized_rec = serde_json::to_string(&self.snapshots)?;

        std::fs::write(path, serialized_rec)?;

        Ok(())
    }
}
