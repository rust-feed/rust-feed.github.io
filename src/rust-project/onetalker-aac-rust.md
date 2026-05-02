# OneTalker แอปพลิเคชัน AAC ตัวแรกของโลกที่ขับเคลื่อนด้วย Rust

> 📅 วันที่เผยแพร่: 2026-01-27

สำหรับนักพัฒนาซอฟต์แวร์ส่วนใหญ่ "ความเร็ว" อาจเป็นเพียงตัวชี้วัดประสิทธิภาพ แต่สำหรับ Gavin Henry และ Ben (ลูกชายของเขา) ความเร็วเป็นอะไรที่มากกว่านั้น Ben นั้นเป็นผู้พิการที่ต้องใช้วีลแชร์และมีภาวะ Quadriplegic Cerebral Palsy — เขาคือผู้ใช้งานที่ต้องการแอปสื่อสารซึ่ง "ห้ามช้า และห้ามแครชโดยเด็ดขาด"

ปัญหาของแอป AAC (Augmentative and Alternative Communication) ในตลาดปัจจุบันคือมักสร้างขึ้นด้วยเทคโนโลยีที่มี Garbage Collector หรือ Web-based runtimes ซึ่งมาพร้อมกับความหน่วง (Latency) ที่คาดเดาไม่ได้ สำหรับ Ben แล้ว เสี้ยววินาทีที่แอปกระตุก คือเสี้ยววินาทีที่เขาเสียสิทธิ์ในการพูด Gavin Henry จึงตัดสินใจสร้าง OneTalker ขึ้นมาใหม่ทั้งหมดโดยยึดสมมติฐานทางวิศวกรรมเพียงข้อเดียว: ถ้าต้องการความเสถียรและความเร็วระดับสูงสุด ต้องใช้ `Rust`

รากฐานของ OneTalker ถูกวางไว้บนสถาปัตยกรรมที่เน้นความปลอดภัยของหน่วยความจำ (Memory Safety) และประสิทธิภาพแบบ Native — เขาเลือกปฏิเสธ Web-based UI frameworks อย่าง Electron/Tauri และหันมาใช้ `iced` (v0.14) เป็นหัวใจหลักในการเรนเดอร์ หากเปิดดู `Cargo.toml` จะพบว่าโปรเจกต์เปิดใช้งานฟีเจอร์ของ Iced เช่น `advanced`, `image` และ `svg` เพื่อรองรับกราฟิกที่ซับซ้อน

แต่สิ่งที่สำคัญกว่ากราฟิกคือกระบวนการจัดการ State: `iced` บังคับใช้ The Elm Architecture อย่างเคร่งครัด โค้ดใน `src/app_state.rs` และไฟล์ภายใต้ `src/screen/` ใช้ `Message` enum ในการส่งเหตุการณ์ ซึ่งทำให้ Data Flow เป็นแบบ Unidirectional และลดความเสี่ยงของ Race Condition ใน UI Thread — พฤติกรรมแบบนี้ช่วยป้องกันแครชที่ยากจะยอมรับได้ในการใช้งาน AAC จริง

เรื่อง Asset และการแจกจ่าย (Distribution) ก็ถูกออกแบบให้เป็น Native จริงจัง: แทนที่จะโหลดไฟล์ไอคอนเวลารัน เขาใช้ `build.rs` ร่วมกับไลบรารี `iced_fontello` เพื่อคอมไพล์ `assets/fonts/icons.toml` เป็น Rust code และฝังไอคอนเหล่านั้นลงใน Binary ตั้งแต่ Compile time สำหรับ Windows ใช้ `embed-resource` และ `windows_exe_info` เพื่อลิงก์ `onetalker.rc` เข้ากับ Executable ทำให้ไฟล์ `.exe` มี Metadata และไอคอนฝังในตัว ลดความเสี่ยงเรื่อง Missing Assets เมื่อแจกจ่ายแอป

ส่วนที่เป็นหัวใจจริงๆ ของแอปคือระบบเสียง (TTS) — OneTalker ต้องการ TTS ที่ฟังดูเป็นธรรมชาติและทำงานแบบ Offline 100% เพื่อหลีกเลี่ยง Latency เครือข่าย ทีมงานผสมผสาน `piper-rs`, `rodio` และ `ort` (ONNX Runtime) เพื่อรันโมเดลสังเคราะห์เสียงบนเครื่องผู้ใช้โดยตรง การรัน Inference ของโมเดลเป็นงานหนักที่อาจทำให้ UI ค้างได้ ดังนั้นการใช้ `tokio` เพื่อรันงาน Inference ใน Background Thread แยกต่างหาก ช่วยให้ UI ตอบสนองได้ทันทีแม้ในขณะที่กำลังสังเคราะห์เสียง

การผสานระหว่าง Ownership ของ Rust กับ C/C++ bindings ของ ONNX ผ่าน `ort` ช่วยให้การจัดการ Buffer ของข้อมูลเสียงปลอดภัย — ข้อมูลเสียงจะไม่ถูก Drop ก่อน `rodio` เล่นจบ และ Memory Safety ช่วยลดความเสี่ยงของ Use-After-Free ที่อาจทำให้แอปแครชได้

เพื่อให้โครงสร้างโปรเจกต์ยั่งยืน OneTalker ถูกจัดเป็น Cargo Workspace แยก `crates/` ออกเป็นส่วนย่อย เช่น `crates/open_board_format` ซึ่ง Implement มาตรฐาน OBF (Open Board Format) สำหรับแลกเปลี่ยนบอร์ดสื่อสาร โดยใช้ `serde`/`serde_json` สำหรับ Serialization ทำให้ส่วนต่างๆ สามารถนำไปใช้ต่อในโปรเจกต์ Accessibility อื่นๆ ได้ง่าย

สรุป: OneTalker เป็นกรณีตัวอย่างที่ชัดเจนว่าเมื่อความหน่วงและความเสถียรคือเรื่องชีวิตและความตายของผู้ใช้ การเลือก `Rust` เป็นพื้นฐานช่วยให้ทีมออกแบบระบบที่ปลอดภัย, เร็ว, และสามารถจัดการงานระดับ Native ได้โดยไม่ต้องแลกกับผู้ใช้ด้วยประสบการณ์ที่ไม่สม่ำเสมอ

**Credit & Reference:**

1. [Website : https://onetalker.org/](https://onetalker.org/)
2. [Source Code : https://codeberg.org/OneTalker/OneTalker](https://codeberg.org/OneTalker/OneTalker)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
