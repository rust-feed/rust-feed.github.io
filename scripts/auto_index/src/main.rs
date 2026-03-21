use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

// Let's assume the script is always run from the root of `rust-feed`.
const ROOT_SRC_DIR: &str = "src";

fn discover_categories(src_path: &Path) -> Vec<(String, String)> {
    let mut categories = Vec::new();

    if let Ok(entries) = fs::read_dir(src_path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().into_owned();

                if folder_name == "assets" || folder_name.starts_with('.') {
                    continue;
                }

                let display_name = folder_name
                    .split('-')
                    .map(|word| {
                        let mut c = word.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ");

                categories.push((display_name, folder_name));
            }
        }
    }

    categories.sort_by(|a, b| a.1.cmp(&b.1));
    categories
}

/// Extract the title from the first `# ` heading in a markdown file.
fn extract_title(filepath: &Path) -> String {
    if let Ok(file) = fs::File::open(filepath) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if let Some(stripped) = line.strip_prefix("# ") {
                return stripped.trim().to_string();
            }
        }
    }
    // Fallback to filename
    filepath
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// Extract the published date from a line matching `> 📅 วันที่เผยแพร่: YYYY-MM-DD`.
/// Returns `None` if no such line is found.
fn extract_date(filepath: &Path) -> Option<String> {
    if let Ok(file) = fs::File::open(filepath) {
        let reader = io::BufReader::new(file);
        for line in reader.lines().map_while(Result::ok) {
            if let Some(rest) = line.strip_prefix("> 📅 วันที่เผยแพร่: ")
            {
                let date = rest.trim().to_string();
                if date.len() >= 10 {
                    return Some(date[..10].to_string());
                }
            }
        }
    }
    None
}

/// Article data: title, filename, and optional date.
#[derive(Debug)]
struct Article {
    title: String,
    filename: String,
    date: Option<String>,
}

fn generate_index_content(category_name: &str, articles: &[Article]) -> String {
    let mut content = format!("# {}\n\n", category_name);
    if articles.is_empty() {
        content.push_str("ยังไม่มีบทความในหมวดหมู่นี้\n");
    } else {
        for article in articles {
            if let Some(ref date) = article.date {
                content.push_str(&format!(
                    "- [{}](./{}) — {}\n",
                    article.title, article.filename, date
                ));
            } else {
                content.push_str(&format!("- [{}](./{})\n", article.title, article.filename));
            }
        }
    }
    content
}

type CategoriesData<'a> = &'a [(String, String, Vec<Article>)];

fn generate_summary_content(categories_data: CategoriesData) -> String {
    let mut content = String::from(
        "<!-- markdownlint-disable MD025 -->\n\n# Summary\n\n[Introduction](./README.md)\n\n",
    );

    for (category_name, folder_name, articles) in categories_data {
        content.push_str(&format!("# {}\n\n", category_name));
        content.push_str(&format!("- [{category_name}](./{folder_name}/index.md)\n"));

        for article in articles {
            content.push_str(&format!(
                "  - [{}](./{}/{})\n",
                article.title, folder_name, article.filename
            ));
        }
        content.push('\n');
    }
    // Trim trailing whitespace to avoid multiple consecutive blank lines (MD012)
    let trimmed = content.trim_end().to_string();
    trimmed + "\n"
}

/// Sort articles by date descending (newest first).
/// Articles without a date are placed at the end, sorted by filename.
fn sort_articles_by_date(articles: &mut [Article]) {
    articles.sort_by(|a, b| match (&b.date, &a.date) {
        (Some(date_b), Some(date_a)) => date_b.cmp(date_a),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.filename.cmp(&b.filename),
    });
}

fn main() {
    let src_path = Path::new(ROOT_SRC_DIR);
    if !src_path.exists() {
        eprintln!(
            "Error: {} directory not found. Please run this command from the root of the project.",
            ROOT_SRC_DIR
        );
        std::process::exit(1);
    }

    let mut categories_data = Vec::new();

    let discovered_categories = discover_categories(src_path);

    for (cat_name, cat_folder) in discovered_categories {
        let folder_path = src_path.join(&cat_folder);

        let mut articles = Vec::new();

        if let Ok(entries) = fs::read_dir(&folder_path) {
            let mut files: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            // Sort files by name first for consistent initial ordering
            files.sort_by_key(|e| e.file_name());

            for entry in files {
                let path = entry.path();
                if path.is_file() {
                    let file_name = entry.file_name().to_string_lossy().into_owned();
                    if file_name.ends_with(".md") && file_name != "index.md" {
                        let title = extract_title(&path);
                        let date = extract_date(&path);
                        articles.push(Article {
                            title,
                            filename: file_name,
                            date,
                        });
                    }
                }
            }
        }

        // Sort articles by date (newest first)
        sort_articles_by_date(&mut articles);

        let index_content = generate_index_content(&cat_name, &articles);
        let index_path = folder_path.join("index.md");
        if let Err(e) = fs::write(&index_path, index_content) {
            eprintln!("Failed to write {}: {}", index_path.display(), e);
        }

        categories_data.push((cat_name, cat_folder, articles));
    }

    // Generate SUMMARY.md
    let summary_content = generate_summary_content(&categories_data);
    let summary_path = src_path.join("SUMMARY.md");
    if let Err(e) = fs::write(&summary_path, summary_content) {
        eprintln!("Failed to write {}: {}", summary_path.display(), e);
    } else {
        println!("Successfully generated index.md files and updated SUMMARY.md");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_sort_articles_newest_first() {
        let mut articles = vec![
            Article {
                title: "A".into(),
                filename: "a.md".into(),
                date: Some("2026-01-01".into()),
            },
            Article {
                title: "B".into(),
                filename: "b.md".into(),
                date: Some("2026-03-21".into()),
            },
        ];
        sort_articles_by_date(&mut articles);
        assert_eq!(articles[0].filename, "b.md");
    }

    #[test]
    fn test_extract_title() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "# My Test Title\nSome content...").unwrap();

        let title = extract_title(file.path());
        assert_eq!(title, "My Test Title");
    }

    #[test]
    fn test_extract_date() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "> 📅 วันที่เผยแพร่: 2026-03-21\nSome content...").unwrap();

        let date = extract_date(file.path());
        assert_eq!(date.unwrap(), "2026-03-21");
    }
}
