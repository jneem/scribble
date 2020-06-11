use druid::commands;
use druid::platform_menus;
use druid::{
    Command, FileDialogOptions, FileSpec, KeyCode, LocalizedString, MenuDesc, MenuItem, SysMods,
};

use crate::cmd;
use crate::editor_state::{CurrentAction, MaybeSnippetId};
use crate::widgets::ToggleButtonState;

const SCRIBL_FILE_TYPE: FileSpec = FileSpec::new("Scribl animation (.scb)", &["scb"]);
const EXPORT_FILE_TYPE: FileSpec = FileSpec::new("mp4 video (.mp4)", &["mp4"]);

use crate::app_state::AppState;
use crate::editor_state::EditorState;

fn file_menu(data: &EditorState) -> MenuDesc<AppState> {
    let has_path = data.save_path.is_some();

    let new = platform_menus::win::file::new();

    let open = MenuItem::new(
        LocalizedString::new("common-menu-file-open"),
        Command::new(
            commands::SHOW_OPEN_PANEL,
            FileDialogOptions::new().allowed_types(vec![SCRIBL_FILE_TYPE]),
        ),
    )
    .hotkey(SysMods::Cmd, "o");

    let save_as_command = Command::new(
        commands::SHOW_SAVE_PANEL,
        FileDialogOptions::new().allowed_types(vec![SCRIBL_FILE_TYPE]),
    );
    let save_command = if has_path {
        Command::new(commands::SAVE_FILE, None)
    } else {
        save_as_command.clone()
    };
    let save = MenuItem::new(LocalizedString::new("common-menu-file-save"), save_command)
        .hotkey(SysMods::Cmd, "s");

    let save_as = MenuItem::new(
        LocalizedString::new("common-menu-file-save-as"),
        save_as_command,
    );

    // Note that we're reusing the SHOW_SAVE_PANEL command for exporting. There doesn't appear to
    // be another way to get the system file dialog.
    let export = MenuItem::new(
        LocalizedString::new("scribl-menu-file-export").with_placeholder("Export"),
        Command::new(
            commands::SHOW_SAVE_PANEL,
            FileDialogOptions::new().allowed_types(vec![EXPORT_FILE_TYPE]),
        ),
    )
    .hotkey(SysMods::Cmd, "e");

    let close = MenuItem::new(
        LocalizedString::new("common-menu-file-close"),
        commands::CLOSE_WINDOW,
    )
    .hotkey(SysMods::Cmd, "q");

    MenuDesc::new(LocalizedString::new("common-menu-file-menu"))
        .append(new)
        .append(open)
        .append(save)
        .append(save_as)
        .append(export)
        .append_separator()
        .append(close)
}

fn edit_menu(data: &EditorState) -> MenuDesc<AppState> {
    let undo = platform_menus::common::undo().disabled_if(|| !data.undo.borrow().can_undo());
    let redo = platform_menus::common::redo().disabled_if(|| !data.undo.borrow().can_redo());

    let draw = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-draw").with_placeholder("Draw"),
        cmd::DRAW,
    );
    let draw = if data.action.rec_toggle() == ToggleButtonState::ToggledOff {
        draw.hotkey(SysMods::None, KeyCode::Space)
    } else {
        draw.disabled()
    };

    let talk = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-talk").with_placeholder("Talk"),
        cmd::TALK,
    );
    let talk = if data.action.rec_audio_toggle() == ToggleButtonState::ToggledOff {
        talk.hotkey(SysMods::Shift, KeyCode::Space)
    } else {
        talk.disabled()
    };

    let play = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-play").with_placeholder("Play"),
        cmd::PLAY,
    );
    let play = if data.action.play_toggle() == ToggleButtonState::ToggledOff {
        play.hotkey(SysMods::None, "p")
    } else {
        play.disabled()
    };

    let stop = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-stop").with_placeholder("Stop"),
        cmd::STOP,
    );
    // The stop hotkey matches the hotkey that was used to start the current action.
    let stop = match data.action {
        CurrentAction::Playing => stop.hotkey(SysMods::None, "p"),
        CurrentAction::Recording(_) | CurrentAction::WaitingToRecord(_) => {
            stop.hotkey(SysMods::None, KeyCode::Space)
        }
        CurrentAction::RecordingAudio(_) => stop.hotkey(SysMods::Shift, KeyCode::Space),
        _ => stop.disabled(),
    };

    let mark = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-mark").with_placeholder("Set mark"),
        Command::new(cmd::SET_MARK, None),
    )
    .hotkey(SysMods::Cmd, KeyCode::KeyM);

    let warp = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-warp").with_placeholder("Warp snippet"),
        cmd::LERP_SNIPPET,
    )
    .hotkey(SysMods::Cmd, KeyCode::KeyW)
    .disabled_if(|| data.mark.is_none());

    let trunc = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-truncate").with_placeholder("Truncate snippet"),
        cmd::TRUNCATE_SNIPPET,
    )
    .hotkey(SysMods::Cmd, KeyCode::KeyT)
    .disabled_if(|| data.selected_snippet.is_none());

    let delete = MenuItem::new(
        LocalizedString::new("scribl-menu-edit-delete").with_placeholder("Delete selected"),
        Command::new(cmd::DELETE_SNIPPET, MaybeSnippetId::None),
    )
    .hotkey(SysMods::None, KeyCode::Delete)
    .disabled_if(|| data.selected_snippet.is_none());

    MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
        .append(undo)
        .append(redo)
        .append_separator()
        .append(draw)
        .append(talk)
        .append(play)
        .append(stop)
        .append_separator()
        .append(mark)
        .append(warp)
        .append(trunc)
        .append(delete)
}

pub fn make_menu(data: &EditorState) -> MenuDesc<AppState> {
    MenuDesc::empty()
        .append(file_menu(data))
        .append(edit_menu(data))
}