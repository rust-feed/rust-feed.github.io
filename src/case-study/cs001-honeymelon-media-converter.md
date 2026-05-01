# การสร้าง Media Converter ด้วย Rust

> 📅 วันที่เผยแพร่: 2026-02-06

มี Case Study ที่น่าสนใจเกี่ยวกับการ Rewrite Desktop App จาก Electron มาเป็น Tauri (Rust Backend + Vue Frontend) ซึ่งเป็นตัวอย่างที่ชัดเจนมากว่าทำไม Rust ถึงเป็นภาษาที่ใช่สำหรับการทำ System Utility ที่ต้องการประสิทธิภาพสูง และมี Memory Safety

โปรเจกต์นี้ชื่อว่า "Honeymelon" เป็น Media Converter สำหรับ macOS (Apple Silicon) โดยเฉพาะ ซึ่งผู้พัฒนา (Jerome Thayananthajothy) ได้แชร์บทเรียนสำคัญในการทิ้ง Node.js runtime อันเทอะทะ มาสู่ความเร็วระดับ Native ของ Rust

## The Cost of Runtime ทำไมต้องหนีจาก Electron?

ปัญหาคลาสสิกของ Electron คือ Resource Overhead ครับ ผู้พัฒนาพบว่าเวอร์ชันแรกกินแรมมหาศาล (Unreasonable memory consumption) และ Binary มีขนาดใหญ่ (Bloated binary) การเปิด App ขึ้นมาเพื่อแปลงไฟล์ง่ายๆ แต่ต้องรัน Chromium ทั้งตัวเป็นเรื่องที่ Overkill เกินไป

เมื่อย้ายมาใช้ Tauri 2 (Rust Backend) ผลลัพธ์คือ:

- **Startup Time แทบจะ Instant** (Rust init FFmpeg capability detection แบบ Background thread ขณะที่ UI render)
- **Memory Usage ลดลงเหลือระดับ Native Utility** ไม่ใช่ Web Browser
- **App Size DMG มีขนาดเล็กลงอย่างมหาศาล**

## Rust Async Runtime vs Node.js Event Loop

จุดที่น่าสนใจที่สุดคือการจัดการ Concurrency ครับ ใน Electron การจัดการ Child Process จำนวนมากผ่าน Node.js Event Loop มักจะมี Overhead ในการ Marshalling ข้อมูลผ่าน Single Thread แต่ในฝั่ง Rust Backend ของ Honeymelon ใช้ประโยชน์จาก Rust’s Async Runtime ร่วมกับ Tauri’s IPC layer ได้อย่างเต็มประสิทธิภาพ

สามารถ handle Concurrent FFmpeg processes ได้โดยไม่มี Overhead เหมือน Node.js และ Resource Management มีการ Implement Logic ที่เรียกว่า "Exclusive mode" สำหรับ Codec ที่กินทรัพยากรสูง (เช่น AV1 หรือ ProRes) ระบบจะ Lock ไม่ให้จ็อบหนักๆ รันซ้อนกันเพื่อป้องกัน Resource Contention ในขณะที่จ็อบเบาๆ สามารถรันขนานกันได้

## สิ่งที่ Rust มอบให้

การเลือก Rust ไม่ใช่แค่เรื่องความเร็ว แต่คือ **Reliability** เพราะ:

- **No Null Pointer Panics:** บอกลา `undefined is not a function` หรือ Crash กลางอากาศขณะแปลงไฟล์
- **Typed Result Values:** การใช้ `Result<T, E>` ทำให้ Error Propagation ถูกจัดการอย่างถูกต้องและครอบคลุมทุกเคส
- **No GC Pauses:** เรื่องนี้สำคัญมากสำหรับงาน Real-time progress tracking เพราะ Rust ไม่มี Garbage Collector มาคอย interrupt การทำงาน ทำให้ Progress events ที่ส่งกลับไปหน้าบ้านมีความลื่นไหลและแม่นยำ

## Architecture Design Probe, Plan, Execute

สถาปัตยกรรมของ Honeymelon แบ่งเป็น 3 Stage ที่น่าสนใจ โดยมีการแบ่งหน้าที่ระหว่าง Frontend (TS) และ Backend (Rust) อย่างชัดเจน:

- **Stage 1 - Probe (Rust):** Backend สั่ง `ffprobe` เพื่อดึง Metadata (Codec, Resolution, Color Primaries ฯลฯ)
- **Stage 2 - Plan (TypeScript):** Logic การตัดสินใจว่าจะ "Remux" (Copy stream) หรือ "Transcode" ย้ายมาทำที่ Frontend ทั้งหมด เพื่อลด Round-trip ไปที่ Rust backend ทำให้ User เห็น Plan ทันที (Zero Latency UX)
- **Stage 3 - Execute (Rust):** พระเอกของงาน Rust จะ spawn `ffmpeg` เป็น Child process และ Dedicated Thread มีการแยก Thread ออกมาเพื่อ parse `stderr` output ของ FFmpeg แบบ Real-time เพื่อคำนวณ % progress, fps, speed จากนั้น Atomic Writes เพื่อความปลอดภัยของข้อมูล Output file จะถูกเขียนลง Temporary path ก่อน และใช้ Atomic Rename เมื่อ process เสร็จสมบูรณ์ ป้องกันไฟล์เสียหากโปรแกรม Crash หรือถูก Cancel กลางคัน

## การจัดการเรื่อง License

หากใครที่เคยทำงานกับ FFmpeg จะรู้ว่าเรื่อง License (LGPL/GPL) นั้นปวดหัว การ Link library (`libavcodec`) เข้ากับ Rust code ตรงๆ อาจทำให้ Binary ของเราติดเงื่อนไข LGPL ซึ่ง Honeymelon แก้ไขด้วย **Process Separation** ไม่ใช้การ Dynamic Linking หรือ Library Calls ใช้ Rust รัน FFmpeg เป็น Separate Process สื่อสารผ่าน Command-line args, Standard Streams และ File System เท่านั้น วิธีนี้ทำให้ Code หลักของ Honeymelon สะอาด (Clean) และสามารถ Release ภายใต้ GPL v3 ได้โดยไม่ขัดแย้งกับ License ของ Libraries

## บทสรุป

Honeymelon พิสูจน์ให้เห็นว่า "The tools you choose determine the experience" การใช้ Rust คู่กับ Tauri ไม่ใช่แค่ Trend แต่มันคือการปลดล็อกข้อจำกัดทาง Performance ที่ Web Stack ทั่วไปทำไม่ได้ โดยเฉพาะงานที่ต้องยุ่งกับ System Process หนักๆ อย่าง Media Conversion

ใครสนใจลองไปแกะ Source Code ดูครับ เขียนด้วย Rust + Vue 3 ออกแบบมาสำหรับ macOS Apple Silicon โดยเฉพาะ เป็นตัวอย่างที่ดีมากสำหรับการจัดการ Command Execution และ Async ใน Rust ครับ

---

**Credit & Reference:**

1. ["Introducing Honeymelon: A Case Study in Building a Better Media Converter" by Jerome Thayananthajothy](https://dev.to/thavarshan/introducing-honeymelon-a-case-study-in-building-a-better-media-converter-51d9)
2. [Honeymelon GitHub repo](https://github.com/honeymelon-app/honeymelon)
3. [Honeymelon Website](https://honeymelon.app/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
