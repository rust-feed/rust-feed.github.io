# Lambda From Scratch: เขียน Custom Runtime เองด้วย Rust แบบไม่ง้อ SDK

> 📅 วันที่เผยแพร่: 2026-02-28

ปกติเวลาเราเขียน AWS Lambda ด้วย Rust เรามักจะจบที่ crate `lambda_runtime` หรือใช้ `cargo-lambda` ซึ่งมันสะดวกมาก แต่เคยสงสัยไหมครับว่า "under the covers" ของ Lambda Runtime จริงๆ แล้วมันคุยกับ AWS Infrastructure อย่างไร

มี Case Study ที่น่าสนใจมากครับ เป็นการเขียน Lambda แบบ "Stripped down" สุดๆ ใช้แค่ crate `reqwest` และ `serde_json` เท่านั้น เพื่อ Implement Lambda Runtime API ด้วยตัวเอง โพสต์นี้ผมจะพาไปดู Mechanism การทำงานระดับ Low-level ของ Serverless และเทคนิคการ Build Rust ให้เป็น bootstrap binary ที่ทรงพลังครับ

---

## 1. Lambda is just a loop การเข้าใจ Abstraction เป็นเรื่องสำคัญ

จริงๆ แล้ว Lambda ไม่ได้ถูกเรียก (Push) โดยตรงเสมอไป แต่มันทำงานผ่าน Polling Model ภายใน Execution Environment ซึ่ง Environment ของ AWS จะ Inject Environment Variable ตัวสำคัญมาให้คือ `AWS_LAMBDA_RUNTIME_API` หน้าที่ของ Binary ของเรา (ในฐานะ Custom Runtime) มีแค่:

- **Init Phase:** จอง Memory, Connect DB (ทำนอก loop `main()`)
- **Processing Loop:** `GET` request ไปยัง `http://{runtime_api}/.../invocation/next` (ขั้นตอนนี้ Runtime จะ block จนกว่าจะมี Event เข้ามา หรือโดน Freeze) Process logic และ `POST` response กลับไปที่ endpoint เดิม

ความสวยงามของ **Rust** คือเราสามารถควบคุม Flow นี้ได้ 100% โดยลด abstraction layer ของ language runtime ลงจนเหลือ interaction กับ Lambda Runtime API โดยตรง แทนที่จะใช้ Macro `#[lambda_runtime::main]` เราเขียน `fn main()` ธรรมดาเลย

> **Note:**
> การจัดการ Error handling และ Retries ในจังหวะ Polling เป็นสิ่งที่ต้อง Handle เองหากทำ Custom Runtime (ตามตัวอย่าง (Ref) มีการทำ Retry logic 3 ครั้งหาก Fetch ไม่สำเร็จ)

---

## 2. จุดที่ Rust กินขาดคือเรื่อง Static Linking และ Binary Size

เพื่อให้รันบน Amazon Linux (AL2/AL2023) ได้โดยไม่ต้องปวดหัวกับ glibc version mismatch เทคนิคมาตรฐานคือการ compile target `x86_64-unknown-linux-musl`

```bash
# Build release for MUSL
cargo build --release --target x86_64-unknown-linux-musl

# Rename to 'bootstrap'
# AWS Custom Runtime มองหาไฟล์ executable ชื่อนี้เท่านั้น
cp target/x86_64-unknown-linux-musl/release/lambda_impl bootstrap

# Strip Symbols (Optional but recommended)
strip bootstrap
```

ผลลัพธ์ที่ได้คือ Binary เดี่ยวๆ (Single Binary) ที่มีขนาดเล็กมาก (Minimal footprint) ไม่มี Dependencies ภายนอก และช่วยลด overhead ของ runtime layer และมีศักยภาพในการทำ cold start ให้เร็วขึ้น

---

## 3. บทสรุป

การเขียนแบบนี้อาจจะไม่เหมาะกับ Production ทั่วไป (ใช้ crate มาตรฐานเถอะครับ 😅 ปลอดภัยกว่าเรื่อง Edge cases) แต่ในเชิงวิศวกรรม มันทำให้เห็นว่า:

- **Memory Safety:** รัสต์ (**Rust**) การันตี memory safety แม้เราจะเขียน low-level interaction เอง
- **Performance:** เราตัด Middleware ที่ไม่จำเป็นออกได้ทั้งหมด
- **Cross-Compilation:** Rust toolchain จัดการเรื่อง Cross-compile ไป MUSL ได้เนียนตาที่สุดภาษาหนึ่งเลยครับ

ใครที่กำลังมองหา High-performance Serverless หรือต้องการ Optimize Cost/Latency สูงสุด **Rust** เป็นตัวเลือกที่ดีมากสำหรับ use case ที่ต้องการควบคุม latency และ footprint สูงสุด

---

**Credit & Reference:**

1. [Lambda From Scratch](https://forgestream.idverse.com/blog/20260119-lambda-from-scratch/)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
