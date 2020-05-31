use crate::{
    app::App,
    logic::{error::AppError, play::play_vn},
    message::Message,
    ui::{add_vn::view::AddVnInput, content::Content, header::Category},
};
use anyhow::Result;
use iced::Command;
use vndb_rs::common::{error::VndbError, get::vn::VN_FLAGS};

pub(crate) fn handle_message(
    app: &mut App,
    message: Message,
) -> Result<Command<Message>> {
    log::debug!("Message: {:?}", message);
    match message {
        Message::Vn(vn) => {
            if let Content::Vn(ref mut vn_content) =
                app.contents.get_current_content()?
            {
                vn_content.update_view(vn);
            }
        }
        Message::Header(category) => {
            app.contents.set_current_content(category)?;
        }
        Message::AddVnInputChanged(input, add_vn_input) => {
            if let Content::AddVn(add_vn_content) =
                app.contents.get_current_content()?
            {
                match add_vn_input {
                    AddVnInput::VnName => add_vn_content.set_vn_id(input),
                    AddVnInput::ExecutablePath => {
                        add_vn_content.set_executable_path(input)
                    }
                }
            }
        }
        Message::AddVn(vn_id, executable_path) => {
            if let Content::AddVn(ref mut content) =
                app.contents.get_content(Category::AddVn)?
            {
                content.clear_msg();
            }
            if let Content::Vn(ref mut vn_content) =
                app.contents.get_content(Category::Vns)?
            {
                let id = match vn_id.parse() {
                    Ok(i) => i,
                    Err(_) => return Err(AppError::VnInvalidId(vn_id).into()),
                };
                if vn_content.vn_exists(id) {
                    return Err(AppError::VnExists(id).into());
                }
                match app.vndb_client.get_vn(
                    &VN_FLAGS,
                    format!("(id={})", vn_id),
                    None,
                ) {
                    Ok(response) => {
                        vn_content.add_vn(response, executable_path)?;
                        if let Content::AddVn(ref mut content) =
                            app.contents.get_content(Category::AddVn)?
                        {
                            content.set_success_msg(
                                "Vn added successfully".into(),
                            );
                        }
                    }
                    Err(err) => {
                        return Err(match err {
                            VndbError::Filter {
                                msg: _,
                                field: _,
                                op: _,
                                value: _,
                            } => AppError::VnNotFound(id).into(),
                            _ => err.into(),
                        })
                    }
                }
            }
        }
        Message::PlayVn(path) => {
            play_vn(&path)?;
        }
        Message::Error(msg) => match app.contents.get_current_content()? {
            Content::Vn(ref mut content) => content.set_error_msg(msg),
            Content::AddVn(ref mut content) => {
                content.set_error_msg(msg);
            }
        },
        Message::Empty(_) => (),
        Message::DeleteVn(vn_id) => {
            if let Content::Vn(ref mut content) =
                app.contents.get_current_content()?
            {
                log::info!("Vn deleted: {}", vn_id);
                content.delete_vn(vn_id)?;
            }
        }
    }
    Ok(Command::none())
}
