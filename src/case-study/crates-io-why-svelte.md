# ทำไม crates.io ถึงเลือก Svelte ?

> 📅 วันที่เผยแพร่: 2026-01-28

ตอนผมอ่าน crates.io: development update มีประเด็นที่เตะตาคือการ migrate frontend ไป #Svelte ผมก็สงสัยว่าทำไมเป็น #Svelte ทั้งที่ถ้าพูดถึง JavaScript Framework ยุคนี้ ตัวเลือกยอดฮิตน่าจะเป็น React/Next ไม่ก็ Vue/Nuxt หรือไม่ก็ไปทาง Rust-based เลย ผมก็เลยไปค้นหาข้อมูลมาแล้วก็พบว่า เหตุผลที่ทำไมทีมงานตัดสินใจเลือกทางนี้ ...

ทำไมไม่ใช้ Rust (SSR/Wasm)?
เล่าก่อนเดิม crates.io นั้นใช้ Ember.js (มีใครรู้จักไหมครับ) จากนั้นทีมงานมีการ Evaluate ตัวเลือกอื่นๆแล้วรวมถึง Server-Side Rendering (SSR) ด้วย #Rust (เช่น minijinja) minijinja นั้น HuggingFace ใช้ทำ render LLM chat templates อยู่ครับ แต่สุดท้ายเคาะที่ #Svelte เหตุผลหลักคือ "Developer Experience (DX)" เนื่องจากทีม crates.io มีขนาดเล็ก (Small team size) การเลือกเครื่องมือที่ Dev ไวและจัดการง่ายจึงสำคัญที่สุด ณ บัดนี้ #Svelte ตอบโจทย์ตรงนี้ได้ดีกว่า

MigrationStrategy ไม่ทำ Big-bang และไม่ทำ Micro-frontends
ทีมเลือกวิธี Incremental Migration แต่ปฏิเสธแนวทาง Micro-frontends หรือการฝัง Svelte Component ลงไปใน Ember app เพราะ Complexity การเอา #Svelte ไปรันใน Ember มันแค่เพิ่มความซับซ้อนให้ Stack โดยไม่ช่วยแก้ปัญหาหลักของการย้ายบ้านจริงๆ (เช่นเรื่อง Data loading หรือ Authentication) และในส่วนของ Architecture เขาใช้วิธีแยก Route ใหม่เป็น `/svelte/` รันขนานไปกับ Ember ของเดิมเลย แล้วค่อยๆ ย้ายทีละ Route พอ Feature Parity ครบ ค่อยสลับ switch

เป้าหมายคือ Pixel Perfect (เพื่อ Test Suite)
โจทย์ของการย้ายครั้งนี้ไม่ใช่การ Redesign แต่คือการทำ Svelte App ให้หน้าตาและพฤติกรรมเหมือน Ember ตัวเดิมเป๊ะๆ (1:1) ด้วยเหตุผลคือ Testability — ทีมมี Test Suite เดิมที่แข็งแกร่งอยู่แล้ว โดยเฉพาะ Visual Regression Tests การย้ายโดยไม่เปลี่ยน Design ทำให้ใช้ Test เดิมจับ Bugs ได้ทันที

Case นี้เป็นตัวอย่างที่ดีนะครับ แม้จะเป็นเว็บรวมแพ็กเกจ #Rust แต่ถ้า Frontend Tooling ตัวไหนช่วยให้ทีมเล็กๆทำงานได้เร็วกว่าและ Maintain ง่ายกว่า เขาก็เลือกตัวนั้น (Svelte) โดยไม่ยึดติดว่าจะต้องเป็น Rust Everything และเลือกท่าที่ Simple ที่สุดในการ Migrate เพื่อลดความเสี่ยงครับ

**Credit & Reference:**

1. [Migrate frontend to Svelte 5 #12515](https://github.com/rust-lang/crates.io/issues/12515)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
