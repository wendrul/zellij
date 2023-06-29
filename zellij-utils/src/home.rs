//!
//! # This module contain everything you'll need to access local system paths
//! containing configuration and layouts

use crate::input::theme::Themes;
use crate::{
    consts::{
        SYSTEM_DEFAULT_CONFIG_DIR, SYSTEM_DEFAULT_DATA_DIR_PREFIX, ZELLIJ_DEFAULT_THEMES,
        ZELLIJ_PROJ_DIR,
    },
    errors::prelude::*,
    input::{
        config::{Config, ConfigError},
        layout::Layout,
        options::Options,
    },
};
use clap::{Args, IntoApp};
use clap_complete::Shell;
use directories_next::BaseDirs;
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom, fmt::Write as FmtWrite, io::Write, path::Path, path::PathBuf, process,
};

pub(crate) const CONFIG_LOCATION: &str = ".config/zellij";

#[cfg(not(test))]
/// Goes through a predefined list and checks for an already
/// existing config directory, returns the first match
pub fn find_default_config_dir() -> Option<PathBuf> {
    default_config_dirs()
        .into_iter()
        .filter(|p| p.is_some())
        .find(|p| p.clone().unwrap().exists())
        .flatten()
}

#[cfg(test)]
pub fn find_default_config_dir() -> Option<PathBuf> {
    None
}

/// Order in which config directories are checked
pub(crate) fn default_config_dirs() -> Vec<Option<PathBuf>> {
    vec![
        home_config_dir(),
        Some(xdg_config_dir()),
        Some(Path::new(SYSTEM_DEFAULT_CONFIG_DIR).to_path_buf()),
    ]
}

/// Looks for an existing dir, uses that, else returns a
/// dir matching the config spec.
pub fn get_default_data_dir() -> PathBuf {
    [
        xdg_data_dir(),
        Path::new(SYSTEM_DEFAULT_DATA_DIR_PREFIX).join("share/zellij"),
    ]
    .into_iter()
    .find(|p| p.exists())
    .unwrap_or_else(xdg_data_dir)
}

#[cfg(not(test))]
pub(crate) fn get_default_themes() -> Themes {
    let mut themes = Themes::default();
    for file in ZELLIJ_DEFAULT_THEMES.files() {
        if let Some(content) = file.contents_utf8() {
            match Themes::from_string(content.to_string()) {
                Ok(theme) => themes = themes.merge(theme),
                Err(_) => {},
            }
        }
    }

    themes
}

#[cfg(test)]
pub(crate) fn get_default_themes() -> Themes {
    Themes::default()
}

pub fn xdg_config_dir() -> PathBuf {
    ZELLIJ_PROJ_DIR.config_dir().to_owned()
}

pub fn xdg_data_dir() -> PathBuf {
    ZELLIJ_PROJ_DIR.data_dir().to_owned()
}

pub fn home_config_dir() -> Option<PathBuf> {
    if let Some(user_dirs) = BaseDirs::new() {
        let config_dir = user_dirs.home_dir().join(CONFIG_LOCATION);
        Some(config_dir)
    } else {
        None
    }
}

pub fn get_layout_dir(config_dir: Option<PathBuf>) -> Option<PathBuf> {
    config_dir.map(|dir| dir.join("layouts"))
}

pub fn default_layout_dir() -> Option<PathBuf> {
    find_default_config_dir().map(|dir| dir.join("layouts"))
}

pub fn get_theme_dir(config_dir: Option<PathBuf>) -> Option<PathBuf> {
    config_dir.map(|dir| dir.join("themes"))
}

pub fn default_theme_dir() -> Option<PathBuf> {
    find_default_config_dir().map(|dir| dir.join("themes"))
}
