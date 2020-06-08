use directories_next::ProjectDirs;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};

use crate::editor_state::StatusMsg;
use crate::save_state::SaveFileData;

pub struct AutosaveData {
    pub path: Option<PathBuf>,
    pub data: SaveFileData,
}

impl AutosaveData {
    fn autosave_path(&self) -> Option<PathBuf> {
        if let Some(proj_dirs) = ProjectDirs::from("ink", "scribl", "scribl") {
            let autosave_name =
                if let Some(orig_name) = self.path.as_ref().and_then(|p| p.file_stem()) {
                    let mut name = orig_name.to_owned();
                    name.push(".autosave.scb");
                    name
                } else {
                    OsStr::new("untitled-autosave.scb").to_os_string()
                };
            let mut ret = proj_dirs.data_local_dir().to_owned();
            ret.push(autosave_name);
            Some(ret)
        } else {
            None
        }
    }
}

pub fn spawn_autosave_thread(status: Sender<StatusMsg>) -> Sender<AutosaveData> {
    let (tx, rx) = channel::<AutosaveData>();
    std::thread::spawn(move || {
        while let Ok(autosave) = rx.recv() {
            // We save only the most recent requested file (so as not to fall behind in case saving
            // is really slow, or the autosave interval is really short).
            let autosave = rx.try_iter().last().unwrap_or(autosave);
            if let Some(path) = autosave.autosave_path() {
                let result = autosave.data.save_to_path(&path);
                let _ = status.send(StatusMsg::DoneSaving {
                    path,
                    result,
                    autosave: true,
                });
            } else {
                log::warn!("not autosaving, couldn't determine the path");
            }
        }
    });

    tx
}
