# การสร้าง Media Converter ด้วย Rust

## 1. บทนำ (Introduction)

มี Case Study ที่น่าสนใจเกี่ยวกับการ Rewrite Desktop App จาก Electron มาเป็น Tauri (Rust Backend + Vue Frontend) ซึ่งเป็นตัวอย่างที่ชัดเจนมากว่าทำไม Rust ถึงเป็นภาษาที่ใช่สำหรับการทำ System Utility ที่ต้องการประสิทธิภาพสูง และมี Memory Safety

โปรเจกต์นี้ชื่อว่า **"Honeymelon"** เป็น Media Converter สำหรับ macOS (Apple Silicon) โดยเฉพาะ ซึ่งผู้พัฒนา (Jerome Thayananthajothy) ได้แชร์บทเรียนสำคัญในการทิ้ง Node.js runtime อันเทอะทะ มาสู่ความเร็วระดับ Native ของ Rust

---

## 2. The Cost of Runtime: ทำไมต้องหนีจาก Electron

ปัญหาคลาสสิกของ Electron คือ Resource Overhead ครับ ผู้พัฒนาพบว่าเวอร์ชันแรกกินแรมมหาศาล (Unreasonable memory consumption) และ Binary มีขนาดใหญ่ (Bloated binary) การเปิด App ขึ้นมาเพื่อแปลงไฟล์ง่ายๆ แต่ต้องรัน Chromium ทั้งตัวเป็นเรื่องที่ Overkill เกินไป

> **Pain Points ของ Electron Version**
>
> - **Memory Usage สูงเกินจำเป็น** — รัน Chromium + Node.js ทั้ง Stack เพื่อทำงานง่ายๆ อย่างแปลงไฟล์
> - **Binary Size ใหญ่** — App ที่ควรจะเบา กลับมีขนาดเทอะทะจาก Chromium bundle
> - **Startup Time ช้า** — ต้อง Boot Chromium engine ก่อนถึงจะใช้งานได้
> - **Node.js Event Loop มี Overhead** — การจัดการ Child Process จำนวนมากผ่าน Single Thread มี Overhead ในการ Marshalling ข้อมูล

---

## 3. สิ่งที่ Rust มอบให้: ทำไมถึงเลือก Rust

การเลือก Rust ไม่ใช่แค่เรื่องความเร็ว แต่คือ **Reliability** ที่ภาษาอื่นให้ไม่ได้ในระดับเดียวกัน เมื่อเทียบกับทางเลือกอื่นอย่าง C++ หรือ Go แล้ว Rust ตอบโจทย์ได้ครบทุกด้านสำหรับงาน System Utility แบบนี้

> **คุณสมบัติเฉพาะของ Rust ที่ตอบโจทย์ Honeymelon**
>
> 1. **No Null Pointer Panics (Memory Safety without GC)** — บอกลา `undefined is not a function` หรือ Crash กลางอากาศขณะแปลงไฟล์ Rust รับประกัน Memory Safety ตั้งแต่ Compile time โดยไม่ต้องพึ่ง Garbage Collector
> 2. **Typed Result Values (Type System & Error Handling)** — การใช้ `Result<T, E>` ทำให้ Error Propagation ถูกจัดการอย่างถูกต้องและครอบคลุมทุกเคส ไม่มี Unhandled Exception หลุดไป
> 3. **No GC Pauses (Zero-cost Abstractions)** — สำคัญมากสำหรับงาน Real-time progress tracking เพราะ Rust ไม่มี Garbage Collector มาคอย interrupt ทำให้ Progress events ลื่นไหลและแม่นยำ
> 4. **Fearless Concurrency** — สามารถ handle Concurrent FFmpeg processes ได้อย่างปลอดภัย ระบบ Ownership ของ Rust ป้องกัน Data Race ตั้งแต่ Compile time
> 5. **Tauri Ecosystem** — Rust Backend + Web Frontend ผ่าน Tauri ทำให้ได้ทั้ง Native Performance และ Modern UI โดยไม่ต้องแบก Chromium ทั้งตัว

---

## 4. Architecture Design: Probe, Plan, Execute

สถาปัตยกรรมของ Honeymelon แบ่งเป็น 3 Stage ที่น่าสนใจ โดยมีการแบ่งหน้าที่ระหว่าง Frontend (TS) และ Backend (Rust) อย่างชัดเจน

> **3 Stages of Honeymelon**
>
> **Stage 1 — Probe (Rust)**
> Backend สั่ง `ffprobe` เพื่อดึง Metadata (Codec, Resolution, Color Primaries ฯลฯ)
>
> **Stage 2 — Plan (TypeScript)**
> Logic การตัดสินใจว่าจะ "Remux" (Copy stream) หรือ "Transcode" ย้ายมาทำที่ Frontend ทั้งหมด เพื่อลด Round-trip ไปที่ Rust backend ทำให้ User เห็น Plan ทันที (**Zero Latency UX**)
>
> **Stage 3 — Execute (Rust)**
> พระเอกของงาน — Rust จะ spawn `ffmpeg` เป็น Child process พร้อม Dedicated Thread สำหรับ parse `stderr` output แบบ Real-time เพื่อคำนวณ % progress, fps, speed

---

## 5. Concurrency Model: Exclusive Mode

จุดที่น่าสนใจที่สุดคือการจัดการ Concurrency ครับ ใน Electron การจัดการ Child Process จำนวนมากผ่าน Node.js Event Loop มักจะมี Overhead ในการ Marshalling ข้อมูลผ่าน Single Thread แต่ในฝั่ง Rust Backend ของ Honeymelon ใช้ประโยชน์จาก Rust's Async Runtime ร่วมกับ Tauri's IPC layer ได้อย่างเต็มประสิทธิภาพ

> **Concurrent FFmpeg Processing**
>
> สามารถ handle Concurrent FFmpeg processes ได้โดยไม่มี Overhead เหมือน Node.js และมีการ Implement Logic ที่เรียกว่า **"Exclusive mode"** สำหรับ Codec ที่กินทรัพยากรสูง (เช่น AV1 หรือ ProRes):
>
> - ระบบจะ **Lock** ไม่ให้จ็อบหนักๆ รันซ้อนกันเพื่อป้องกัน Resource Contention
> - ในขณะที่จ็อบเบาๆ สามารถ **รันขนานกัน**ได้

> **Atomic Writes เพื่อความปลอดภัย**
>
> Output file จะถูกเขียนลง **Temporary path** ก่อน และใช้ **Atomic Rename** เมื่อ process เสร็จสมบูรณ์ ป้องกันไฟล์เสียหากโปรแกรม Crash หรือถูก Cancel กลางคัน

---

## 6. FFmpeg License: Process Separation

หากใครที่เคยทำงานกับ FFmpeg จะรู้ว่าเรื่อง License (LGPL/GPL) นั้นปวดหัว การ Link library (`libavcodec`) เข้ากับ Rust code ตรงๆ อาจทำให้ Binary ของเราติดเงื่อนไข LGPL

> **วิธีแก้ปัญหา License อย่างสะอาด**
>
> Honeymelon ใช้ **Process Separation**:
>
> - ❌ ไม่ใช้ Dynamic Linking หรือ Library Calls
> - ✅ ใช้ Rust รัน FFmpeg เป็น **Separate Process**
> - ✅ สื่อสารผ่าน Command-line args, Standard Streams และ File System เท่านั้น
>
> วิธีนี้ทำให้ Code หลักของ Honeymelon สะอาด (Clean) และสามารถ Release ภายใต้ **GPL v3** ได้โดยไม่ขัดแย้งกับ License ของ Libraries

---

## 7. ผลลัพธ์และตัวเลข: Electron vs Tauri (Rust)

การย้ายจาก Electron มา Tauri (Rust Backend) ให้ผลลัพธ์ที่เห็นได้ชัดในทุกด้าน

| Metric | Electron (Before) | Tauri + Rust (After) | ผลลัพธ์ |
|---|---|---|---|
| **Startup Time** | ช้า — ต้อง Boot Chromium | แทบจะ Instant — Rust init FFmpeg detection แบบ Background thread | ⚡ เร็วขึ้นอย่างเห็นได้ชัด |
| **Memory Usage** | สูง — รัน Chromium + Node.js ทั้ง Stack | ระดับ Native Utility | 📉 ลดลงอย่างมหาศาล |
| **App Size (DMG)** | ใหญ่ — Bundle Chromium ทั้งตัว | เล็กลงมาก — ใช้ WebView ของ OS | 📦 ขนาดเล็กลงมาก |
| **Concurrent Processing** | Node.js Event Loop — มี Overhead | Rust Async Runtime — ไม่มี Overhead | 🚀 รองรับ Concurrency ได้ดีกว่า |
| **Crash / Error Rate** | เสี่ยง Null Pointer & Unhandled Exception | Memory Safety + Result Type | 🛡️ Reliable มากขึ้น |
| **Progress Tracking** | GC Pauses อาจทำให้กระตุก | No GC — ลื่นไหลและแม่นยำ | 📊 Real-time ที่แท้จริง |

---

## 8. บทเรียนและข้อควรระวัง

### สิ่งที่ทำแล้วได้ผลดี

- **Process Separation สำหรับ FFmpeg** — แยก FFmpeg เป็น Child Process แทนการ Link library ช่วยแก้ปัญหา License ได้สะอาด และทำให้ Debug ง่ายขึ้น
- **แบ่ง Logic ระหว่าง Frontend/Backend อย่างชัดเจน** — การให้ TypeScript ทำ Planning (Stage 2) ช่วยลด IPC Round-trip และให้ UX ที่ตอบสนองเร็วขึ้น
- **Atomic Writes** — ป้องกันไฟล์เสียหายได้จริงในกรณี Crash หรือ Cancel เป็น Pattern ที่แนะนำให้ใช้ในทุกโปรเจกต์ที่เขียนไฟล์
- **Exclusive Mode สำหรับ Codec หนัก** — การมี Smart Scheduling ที่ Lock จ็อบหนักไม่ให้ซ้อนกัน ช่วยป้องกัน System Resource ล่ม

### สิ่งที่ควรระวัง

- **Tauri ยังเป็น Ecosystem ที่กำลังเติบโต** — Plugin และ Community ยังไม่ใหญ่เท่า Electron ต้องเตรียมใจว่าบางอย่างอาจต้องเขียนเอง
- **macOS Only (Apple Silicon)** — การที่ Target เฉพาะ Platform เดียวทำให้ Optimize ได้ลึก แต่ถ้าจะรองรับ Cross-platform ต้องพิจารณา Platform-specific Code เพิ่มเติม
- **FFmpeg CLI Parsing** — การ parse `stderr` ของ FFmpeg เพื่อ track progress ต้องรับมือกับ Output format ที่อาจเปลี่ยนได้ตาม Version ของ FFmpeg

---

## 9. บทสรุป

Honeymelon พิสูจน์ให้เห็นว่าการใช้ Rust คู่กับ Tauri ไม่ใช่แค่ Trend แต่มันคือการ **ปลดล็อกข้อจำกัดทาง Performance** ที่ Web Stack ทั่วไปทำไม่ได้ โดยเฉพาะงานที่ต้องยุ่งกับ System Process หนักๆ อย่าง Media Conversion

การแบ่ง Architecture เป็น Probe → Plan → Execute ร่วมกับ Rust's Ownership System และ Async Runtime ทำให้ได้ App ที่ทั้งเร็ว เสถียร และจัดการ Resource ได้อย่างชาญฉลาด

ใครสนใจลองไปแกะ Source Code ดูครับ เขียนด้วย Rust + Vue 3 ออกแบบมาสำหรับ macOS Apple Silicon โดยเฉพาะ เป็นตัวอย่างที่ดีมากสำหรับการจัดการ Command Execution และ Async ใน Rust

> **Key Takeaway:** เมื่อ Desktop App ของคุณต้องจัดการ System Process หนักๆ ที่ต้องการทั้ง Performance, Memory Safety และ Reliability — Rust + Tauri คือคำตอบที่พิสูจน์แล้วว่าใช้งานได้จริงใน Production

---

**Credit & Reference:**

1. [Introducing Honeymelon: A Case Study in Building a Better Media Converter](https://dev.to/thavarshan/introducing-honeymelon-a-case-study-in-building-a-better-media-converter-51d9)
2. [honeymelon GitHub repo](https://github.com/honeymelon-app/honeymelon)
3. [Honeymelon Website](https://honeymelon.app/)
