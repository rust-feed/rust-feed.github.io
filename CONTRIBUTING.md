# Contributing Guidelines

First off, thank you for considering contributing to `rust-feed` To keep the repository organized and ensure a pleasant reading experience for everyone, we have established standard guidelines for writing and naming articles.

## 1. Naming Convention

All article files must be named according to their respective category (the folder they belong to). The filename format should consist of a **3-digit zero-padded number**, followed by the **kebab-case title** of the article (`.md`).

For example: `modernize-cobol-with-rust.md`, `sabiql-driverless-tui.md`

Note: Do not include category prefixes (such as `cs`, `dd`, `rp`, etc.) in filenames — place the file in the appropriate `src/<category>/` folder and the indexer will map it automatically.

## 2. Article Frontmatter & Formatting

Every article must start with a **Title (H1)**, immediately followed by the **publication date** formatted as a specific blockquote (`> 📅 วันที่เผยแพร่: ...`). This ensures that the generated metadata is consistent across the entire project.

```markdown
# Your Descriptive Article Title Here (H1)

> 📅 วันที่เผยแพร่: YYYY-MM-DD
```

**Example:**

```markdown
# เมื่อ Vite 8 เดิมพันอนาคตของ Web Tooling ด้วยสถาปัตยกรรมแบบที่ขับเคลื่อนด้วย Rust

> 📅 วันที่เผยแพร่: 2026-03-21

The content of your article starts here...
```

## 3. Content Style Guidelines

- **Subheadings:** When starting a new section, use level 2 (`##`) and level 3 (`###`) headers to break the content into readable, manageable chunks.
- **Emphasis:** Use bold text (`**keyword**`) for important concepts or terms you want to highlight.
- **Lists:** Use bullet points (`-`) for itemized information to make scanning easier. **Avoid using horizontal rules (`---`)** unless strictly necessary.
- **The `#Rust` Tag:** Please omit hashtag symbols (`#`) before keywords in the body text. Just write `Rust` normally to keep the markdown content looking clean and readable like a professional blog post.
- **Credits & References:** At the bottom of your article, always provide links to the original sources or repositories (if applicable) without adding a horizontal rule above it:

  ```markdown
  **Credit & Reference:**

  1. [Source Title or Repository](https://link.com)
  ```

## 4. Submission Workflow

Once your article is complete and the file is properly named:

1. Save your `.md` file inside the appropriate category folder (e.g., `src/case-study/` or `src/rust-project/`).
2. **Do not manually edit** `SUMMARY.md` or any category's `index.md` file.
3. Run our `auto_index` script to automatically map your article and generate the table of contents:

   ```bash
   cargo run --manifest-path scripts/auto_index/Cargo.toml --release
   ```

4. Verify the build locally by running `mdbook serve --open` and checking your changes in the browser.
5. If everything looks good, you are ready to open a Pull Request (PR)
