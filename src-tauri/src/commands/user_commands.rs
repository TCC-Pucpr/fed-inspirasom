use crate::app_states::store_state::StoreState;
use crate::commands::ServiceResult;
use crate::constants::store_keys::{KEY_USER_NAME, KEY_USER_PFP};
use std::option::Option;
use tauri::State;

#[tauri::command]
pub async fn set_user_pfp(
    base64: String,
    store_state: State<'_, StoreState>
) -> ServiceResult<()> {
    store_state.save(KEY_USER_PFP, &base64)?;
    Ok(())
}

#[tauri::command]
pub async fn set_user_name(
    name: String,
    store_state: State<'_, StoreState>
) -> ServiceResult<()> {
    store_state.save(KEY_USER_NAME, &name)?;
    Ok(())
}

#[tauri::command]
pub fn user_pfp(
    store_state: State<'_, StoreState>
) -> Option<String> {
    let res = store_state.retrieve(KEY_USER_PFP);
    if let Ok(s) = res {
        Some(s)
    } else {
        None
    }
}

#[tauri::command]
pub fn user_name(
    store_state: State<'_, StoreState>
) -> Option<String> {
    let res = store_state.retrieve(KEY_USER_NAME);
    if let Ok(s) = res {
        Some(s)
    } else {
        None
    }
}