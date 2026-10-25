// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleMeta {
    pub filename: String,
    pub title: String,
    pub category: i32,
    pub category_name: String,
    pub abstract_text: String,
    pub picture: String,
    pub ai_generated: bool,
    pub date: String,
    pub modified_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ArticleFull {
    pub meta: ArticleMeta,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoInfo {
    pub path: String,
    pub article_count: usize,
    pub git_branch: String,
    pub git_status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitResult {
    pub success: bool,
    pub output: String,
    pub error: String,
}

fn get_repo_dir(custom_path: Option<String>) -> PathBuf {
    if let Some(custom) = custom_path {
        let p = PathBuf::from(custom);
        if p.exists() {
            return p;
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("public/articles").exists() {
            return cwd;
        }
        if let Some(parent) = cwd.parent() {
            if parent.join("public/articles").exists() {
                return parent.to_path_buf();
            }
        }
    }

    PathBuf::from("/Users/glowingstone/WebstormProjects/glowingstone-vue")
}

fn category_to_name(cat: i32) -> String {
    match cat {
        0 => "技术".to_string(),
        1 => "闲聊".to_string(),
        2 => "时事".to_string(),
        3 => "文章/小说".to_string(),
        _ => "其他".to_string(),
    }
}

fn parse_frontmatter(content: &str, filename: &str, mod_time: String) -> (ArticleMeta, String) {
    let mut title = filename.trim_end_matches(".md").to_string();
    let mut category = 0;
    let mut abstract_text = String::new();
    let mut picture = "none".to_string();
    let mut ai_generated = false;
    let mut date = "2026年10月".to_string();
    let mut body = content.to_string();

    let trimmed = content.trim_start();
    if trimmed.starts_with("---") {
        let after_first = &trimmed[3..];
        if let Some(end_idx) = after_first.find("---") {
            let fm_str = &after_first[..end_idx];
            body = after_first[end_idx + 3..].trim_start().to_string();

            for line in fm_str.lines() {
                let line = line.trim();
                if let Some((key, val)) = line.split_once(':') {
                    let k = key.trim().to_lowercase();
                    let v = val.trim().trim_matches('"').trim_matches('\'').trim();
                    match k.as_str() {
                        "title" => {
                            if !v.is_empty() {
                                title = v.to_string();
                            }
                        }
                        "abstract" | "excerpt" => {
                            abstract_text = v.to_string();
                        }
                        "category" => {
                            if let Ok(c) = v.parse::<i32>() {
                                category = c;
                            }
                        }
                        "picture" | "cover" | "image" => {
                            picture = v.to_string();
                        }
                        "ai_generated" | "ai" => {
                            ai_generated = v == "true" || v == "1";
                        }
                        "date" | "time" => {
                            if !v.is_empty() {
                                date = v.to_string();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    let meta = ArticleMeta {
        filename: filename.to_string(),
        title,
        category,
        category_name: category_to_name(category),
        abstract_text,
        picture,
        ai_generated,
        date,
        modified_time: mod_time,
    };

    (meta, body)
}

#[tauri::command]
fn get_repo_info(custom_path: Option<String>) -> Result<RepoInfo, String> {
    let repo_dir = get_repo_dir(custom_path);
    if !repo_dir.exists() {
        return Err(format!("博客仓库路径不存在: {:?}", repo_dir));
    }

    let articles_dir = repo_dir.join("public/articles");
    let mut count = 0;
    if let Ok(entries) = fs::read_dir(&articles_dir) {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "md" {
                    count += 1;
                }
            }
        }
    }

    let branch = Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&repo_dir)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "main".to_string());

    let status = Command::new("git")
        .args(["status", "--short"])
        .current_dir(&repo_dir)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "".to_string());

    Ok(RepoInfo {
        path: repo_dir.to_string_lossy().to_string(),
        article_count: count,
        git_branch: if branch.is_empty() { "main".to_string() } else { branch },
        git_status: status,
    })
}

#[tauri::command]
fn list_articles(custom_path: Option<String>) -> Result<Vec<ArticleMeta>, String> {
    let repo_dir = get_repo_dir(custom_path);
    let articles_dir = repo_dir.join("public/articles");

    if !articles_dir.exists() {
        return Err(format!("文章目录不存在: {:?}", articles_dir));
    }

    let mut list = Vec::new();
    let entries = fs::read_dir(&articles_dir).map_err(|e| e.to_string())?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            let mod_time = entry
                .metadata()
                .and_then(|m| m.modified())
                .map(|t| {
                    let duration = t.duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
                    format!("{}", duration.as_secs())
                })
                .unwrap_or_default();

            if let Ok(content) = fs::read_to_string(&path) {
                let (meta, _) = parse_frontmatter(&content, &filename, mod_time);
                list.push(meta);
            }
        }
    }

    // Sort by modified time descending
    list.sort_by(|a, b| b.modified_time.cmp(&a.modified_time));
    Ok(list)
}

#[tauri::command]
fn get_article(custom_path: Option<String>, filename: String) -> Result<ArticleFull, String> {
    let repo_dir = get_repo_dir(custom_path);
    let file_path = repo_dir.join("public/articles").join(&filename);

    if !file_path.exists() {
        return Err(format!("文章文件不存在: {:?}", file_path));
    }

    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let (meta, body) = parse_frontmatter(&content, &filename, String::new());

    Ok(ArticleFull {
        meta,
        content: body,
    })
}

#[tauri::command]
fn save_article(
    custom_path: Option<String>,
    mut filename: String,
    title: String,
    category: i32,
    abstract_text: String,
    picture: String,
    ai_generated: bool,
    date: String,
    content: String,
) -> Result<String, String> {
    let repo_dir = get_repo_dir(custom_path);
    let articles_dir = repo_dir.join("public/articles");

    if !articles_dir.exists() {
        fs::create_dir_all(&articles_dir).map_err(|e| e.to_string())?;
    }

    if !filename.ends_with(".md") {
        filename.push_str(".md");
    }

    let file_path = articles_dir.join(&filename);

    // Escape quotes in strings for frontmatter
    let clean_title = title.replace('"', "\\\"");
    let clean_abstract = abstract_text.replace('"', "\\\"");
    let clean_pic = if picture.trim().is_empty() {
        "none".to_string()
    } else {
        picture.trim().to_string()
    };
    let clean_date = if date.trim().is_empty() {
        "2026年10月".to_string()
    } else {
        date.trim().to_string()
    };

    let frontmatter = format!(
        "---\ntitle: \"{}\"\nabstract: \"{}\"\npicture: \"{}\"\ncategory: {}\nai_generated: {}\ndate: \"{}\"\n---\n\n{}",
        clean_title, clean_abstract, clean_pic, category, ai_generated, clean_date, content
    );

    fs::write(&file_path, frontmatter).map_err(|e| format!("保存失败: {}", e))?;
    Ok(filename)
}

#[tauri::command]
fn delete_article(custom_path: Option<String>, filename: String) -> Result<bool, String> {
    let repo_dir = get_repo_dir(custom_path);
    let file_path = repo_dir.join("public/articles").join(&filename);

    if file_path.exists() {
        fs::remove_file(&file_path).map_err(|e| format!("删除失败: {}", e))?;
        Ok(true)
    } else {
        Err("文件不存在".to_string())
    }
}

#[tauri::command]
fn save_image_base64(
    custom_path: Option<String>,
    filename: String,
    base64_data: String,
) -> Result<String, String> {
    let repo_dir = get_repo_dir(custom_path);
    let articles_dir = repo_dir.join("public/articles");

    if !articles_dir.exists() {
        fs::create_dir_all(&articles_dir).map_err(|e| e.to_string())?;
    }

    // Strip prefix like "data:image/png;base64," if present
    let raw_b64 = if let Some((_, rest)) = base64_data.split_once(',') {
        rest
    } else {
        &base64_data
    };

    let bytes = BASE64
        .decode(raw_b64.trim())
        .map_err(|e| format!("Base64 解码失败: {}", e))?;

    let file_path = articles_dir.join(&filename);
    fs::write(&file_path, bytes).map_err(|e| format!("写入图片失败: {}", e))?;

    Ok(filename)
}

#[tauri::command]
fn git_status(custom_path: Option<String>) -> Result<String, String> {
    let repo_dir = get_repo_dir(custom_path);
    let output = Command::new("git")
        .args(["status", "--short"])
        .current_dir(&repo_dir)
        .output()
        .map_err(|e| format!("执行 git 失败: {}", e))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn git_publish(custom_path: Option<String>, commit_msg: String) -> Result<GitResult, String> {
    let repo_dir = get_repo_dir(custom_path);

    // 1. git add .
    let add_out = Command::new("git")
        .args(["add", "."])
        .current_dir(&repo_dir)
        .output()
        .map_err(|e| format!("git add 失败: {}", e))?;

    if !add_out.status.success() {
        return Ok(GitResult {
            success: false,
            output: String::from_utf8_lossy(&add_out.stdout).to_string(),
            error: String::from_utf8_lossy(&add_out.stderr).to_string(),
        });
    }

    // 2. git commit -m
    let msg = if commit_msg.trim().is_empty() {
        "publish: new article from Glowstone Publisher".to_string()
    } else {
        commit_msg
    };

    let commit_out = Command::new("git")
        .args(["commit", "-m", &msg])
        .current_dir(&repo_dir)
        .output()
        .map_err(|e| format!("git commit 失败: {}", e))?;

    // 3. git push origin main
    let push_out = Command::new("git")
        .args(["push", "origin", "main"])
        .current_dir(&repo_dir)
        .output()
        .map_err(|e| format!("git push 失败: {}", e))?;

    let combined_out = format!(
        "--- Commit ---\n{}\n--- Push ---\n{}",
        String::from_utf8_lossy(&commit_out.stdout),
        String::from_utf8_lossy(&push_out.stdout)
    );

    let combined_err = format!(
        "{}\n{}",
        String::from_utf8_lossy(&commit_out.stderr),
        String::from_utf8_lossy(&push_out.stderr)
    );

    Ok(GitResult {
        success: push_out.status.success() || commit_out.status.success(),
        output: combined_out,
        error: combined_err,
    })
}

#[tauri::command]
fn open_external_url(url: String) -> Result<bool, String> {
    Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(true)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_repo_info,
            list_articles,
            get_article,
            save_article,
            delete_article,
            save_image_base64,
            git_status,
            git_publish,
            open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
