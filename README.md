# Rust Feed

[![Lint](https://github.com/rust-feed/rust-feed.github.io/actions/workflows/lint.yml/badge.svg)](https://github.com/rust-feed/rust-feed.github.io/actions/workflows/lint.yml)

**Rust Feed** is an open-source collection of Rust articles and tutorials originally posted in the Facebook Rust Dev Community. This repository serves as a centralized, well-organized knowledge base built with [mdBook](https://rust-lang.github.io/mdBook/).

## Access the Book

You can read the compiled book online here:
**[Rust Feed Documentation](https://rust-feed.github.io/)**

---

## Prerequisites

To run this project locally and build the documentation, you will need:

- **Rust and Cargo**: Install via [rustup](https://rustup.rs/) (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- **mdBook**: Install via cargo:

  ```sh
  cargo install mdbook
  ```

---

## How to Contribute / Add New Articles

We have fully automated the indexing process native to the Rust ecosystem. To add a new article to the book:

1. **Create your Markdown file**: Write your article and save it as a `.md` file. Ensure it has a `# Heading 1` at the top as the title.
2. **Place it in the correct category**: Move your `.md` file into the corresponding category folder inside `src/`. For example:

   ```text
   src/case-study/my-new-article.md
   ```

3. **Run the Update Script**: From the root of the project, run the update script. This script will automatically update the category index and regenerate the `SUMMARY.md` file using our custom Rust auto-indexer.

   ```sh
   ./update.sh
   ```

4. **Preview locally** (Optional):

   ```sh
   mdbook serve --open
   ```

---

## Project Structure

```text
rust-feed/
├── book/                  # Compiled HTML output (gitignored)
├── scripts/
│   └── auto_index/        # Custom Rust CLI to auto-generate indices and SUMMARY.md
├── src/                   # Markdown source files for the book
│   ├── case-study/        # Category folders containing articles
│   ├── deep-dive/
│   ├── ...
│   ├── README.md          # Introduction page for mdBook
│   └── SUMMARY.md         # Auto-generated book sidebar structure
├── book.toml              # mdBook configuration
├── update.sh              # Wrapper script to run auto_index and mdbook build
└── README.md              # This file
```

---

## License

This project is licensed under the [MIT License](LICENSE).

---

## Linting

We use `markdownlint-cli2` in CI to lint Markdown files under `src/`.

Run the linter locally (uses `npx` so you don't need a global install):

```bash
npx markdownlint-cli2 "src/**/*.md" --config .markdownlint.yml --fix
```

If you'd like to also run the link checker used in CI, install `lychee` and run:

```bash
cargo install lychee
lychee check --exclude-loopback --exclude "facebook.com" "src/**/*.md"
```

Run the auto-index tests (same as CI):

```bash
cargo test --manifest-path scripts/auto_index/Cargo.toml
```
