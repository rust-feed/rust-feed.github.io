# Assets Directory

This directory contains static assets for the **Rust Feed** mdBook site.

---

## Open Graph Image

### Source File

| Property   | Value            |
|------------|------------------|
| Source      | `og-image.svg`   |
| Production | `og-image.png`   |
| Dimensions | 1200 × 630 px    |
| Format     | SVG → PNG        |

The canonical design lives in `og-image.svg`. Social platforms (Facebook, Twitter/X, LinkedIn) **do not support SVG** for Open Graph images, so you must convert it to PNG before deploying.

---

### SVG → PNG Conversion

Pick whichever method you have available:

#### Option A: Inkscape (CLI)

```sh
inkscape og-image.svg \
  --export-type=png \
  --export-filename=og-image.png \
  --export-width=1200 \
  --export-height=630
```

#### Option B: ImageMagick / `magick`

```sh
magick convert -background none -size 1200x630 og-image.svg og-image.png
```

#### Option C: Chromium / Chrome headless

```sh
chrome --headless --disable-gpu --screenshot=og-image.png \
  --window-size=1200,630 og-image.svg
```

#### Option D: resvg (Rust-native, no dependencies)

```sh
cargo install resvg
resvg og-image.svg og-image.png -w 1200 -h 630
```

#### Option E: Manual

Open `og-image.svg` in any browser, take a 1200×630 screenshot, or use an online tool such as [svgtopng.com](https://svgtopng.com/).

---

### Design Spec

| Element              | Value                                            |
|----------------------|--------------------------------------------------|
| Background           | `#0F1419` (dark, slightly warm)                  |
| Accent color         | `#F74C00` (official Rust orange)                 |
| Title text color     | `#E6EDF3` (high-contrast off-white)              |
| Subtitle text color  | `#6E7681` (muted gray)                           |
| Font stack           | Consolas → Courier New → Liberation Mono → mono  |
| Title size           | 76 px, weight 700                                |
| Subtitle size        | 22 px, letter-spacing 1                          |

**Visual layers (back to front):**

1. Solid `#0F1419` fill
2. Dot grid pattern (`#F74C00` at 7% opacity, 40 px spacing)
3. Scanline texture (white at 1.5% opacity)
4. Radial vignette (black at 55% opacity, edges)
5. Top + bottom accent bars (3 px, orange gradient fading at edges)
6. Ghost curly braces `{ }` flanking content (orange at ~8% opacity)
7. Faint code-line clusters (upper-left, lower-right)
8. Terminal prompt `>` in orange at 50% opacity
9. **"rust-feed"** — centered, white, with subtle orange glow filter
10. Tapering underline accent (orange gradient, 120 px)
11. **"Curated Deep Dives from the Rust Community"** — centered, gray
12. Corner markers (L-shaped, orange at 18% opacity, blueprint aesthetic)

### Safe Zone

Keep all critical text inside the inner **1080 × 560 px** region (60 px inset on each side). Some platforms crop edges when rendering cards.

---

### How It's Referenced

The image is referenced in `theme/head.hbs` via absolute URL:

```text
https://pharmacist-sabot.github.io/rust-feed/assets/og-image.png
```

mdBook copies everything under `src/assets/` into the build output, so both the SVG source and the PNG production file placed here will be served at their respective paths.

> **Important:** The `og-image.png` file **must** exist at deploy time. The SVG alone is not sufficient for social previews.

---

### Post-Deploy Validation

After deploying, verify your tags render correctly:

| Tool | URL |
|------|-----|
| Facebook Sharing Debugger | <https://developers.facebook.com/tools/debug/> |
| Twitter/X Card Validator | <https://cards-dev.twitter.com/validator> |
| OpenGraph.xyz | <https://www.opengraph.xyz/> |
| LinkedIn Post Inspector | <https://www.linkedin.com/post-inspector/> |

Paste `https://pharmacist-sabot.github.io/rust-feed/` and confirm:

- [x] Image renders at 1200 × 630
- [x] Title text is legible at thumbnail size (~600 px wide)
- [x] No critical content is cropped at the edges
- [x] Dark background contrasts well against both light and dark platform UIs
