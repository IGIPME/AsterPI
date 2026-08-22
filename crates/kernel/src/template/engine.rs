use anyhow::{Context, Result, bail};
use include_dir::{Dir, DirEntry, include_dir};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};
use tera::{Context as TeraContext, Tera};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_WORKSPACE_DIR/assets/templates");

/// 模板元数据结构（用于解析 template.toml）
#[derive(Debug, Clone, Deserialize)]
pub struct TemplateMeta {
    pub template: TemplateInfo,
    pub variables: Option<HashMap<String, VariableDef>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub description: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VariableDef {
    pub prompt: Option<String>,
    pub default: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSummary {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwriteMode {
    Fail,
    Overwrite,
    Skip,
}