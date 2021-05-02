use serde::Serialize;

use std::ops::Range;

use crate::project::Project;

#[derive(Serialize)]
pub enum Change {
    CreateDir {
        name: String,
    },
    CreateFile {
        name: String,
        content: String,
    },
    ModifyFile {
        name: String,
        content: String,
        lines: Range<u64>,
    },
}

#[derive(Serialize)]
pub struct Snapshot {
    changes: Vec<Change>,
}

impl Snapshot {
    pub fn new(changes: Vec<Change>) -> Self {
        Self { changes }
    }
}

/// Records a [`Snapshot`](crate::snapshot::Snapshot) and adds it to `project`
pub struct SnapshotRec {
    project: Project,
    changes: Vec<Change>,
}

impl SnapshotRec {
    pub fn new(project: Project) -> Self {
        SnapshotRec {
            project,
            changes: Vec::new(),
        }
    }

    pub fn create_dir<D: ToString>(&mut self, dir_name: D) {
        self.changes.push(Change::CreateDir {
            name: dir_name.to_string(),
        });
    }

    pub fn create_file<F: ToString, C: ToString>(&mut self, file_name: F, content: C) {
        self.changes.push(Change::CreateFile {
            name: file_name.to_string(),
            content: content.to_string(),
        });
    }

    pub fn modify_file<F: ToString, C: ToString>(
        &mut self,
        file_name: F,
        content: C,
        lines: Range<u64>,
    ) {
        self.changes.push(Change::ModifyFile {
            name: file_name.to_string(),
            content: content.to_string(),
            lines,
        });
    }

    pub fn end(self) -> Project {
        let changes = self.changes;
        let mut project = self.project;
        project.snapshots.push(Snapshot::new(changes));
        project
    }
}
