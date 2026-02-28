# เจาะลึกสถาปัตยกรรม "Leptos" กับแนวคิด Fine-Grained Reactivity

การพัฒนา Rust Web Development เรามักคุ้นเคยกับ Framework ที่พยายามยกโมเดลของ React มาใส่ใน **Rust** (เช่น Yew) ซึ่งแม้จะปลอดภัยและเขียนสนุก แต่ลึกๆ แล้วเรายังต้องจ่าย "ภาษี" ให้กับระบบ **Virtual DOM (VDOM)** ไม่ว่าจะเป็นการ Diffing trees หรือการ Re-render component ซ้ำๆ เมื่อ State เปลี่ยน ซึ่งฟังดูแล้วมันขัดแย้งกับปรัชญา **Zero-cost abstractions** ที่ชาว **Rustacean** ยึดถือ

แต่วันนี้ผมอยากพาไปรู้จัก **Leptos** (เล็ปโตส) Framework ที่กล้าทิ้ง VDOM และนำศักยภาพของ **Rust** มาใช้รีดประสิทธิภาพ **Web Assembly (WASM)** จนถึงขีดสุด

---

## 1. ปฏิวัติด้วย Fine-Grained Reactivity

เรื่องราวของ Leptos นั้น เริ่มต้นที่การตั้งคำถามกับ Paradigm เดิมๆ แทนที่จะมองว่า UI คือฟังก์ชันที่ต้องรันซ้ำๆ เพื่อสร้าง Tree ใหม่ (แบบ Yew หรือ Dioxus) **Leptos** เลือกเดินเส้นทางของ **Fine-Grained Reactivity** ซึ่งเปลี่ยนกระบวนการคิดของเราไปโดยสิ้นเชิง

> **Run Once Philosophy**
>
> ใน **Leptos** นั้น Component Function (`#[component]`) จะถูก **รันเพียงแค่ครั้งเดียว (Run once)** ในตอน Initialization เพื่อสร้าง DOM Nodes จริงๆ ขึ้นมา จากนั้นระบบจะสร้าง "Reaction Graph" ผูกติดไว้กับจุดต่างๆ ใน DOM โดยตรง

นั่นหมายความว่า เมื่อ **Signal** มีการเปลี่ยนค่า Runtime ของ **Leptos** ไม่จำเป็นต้อง Traverse ดูว่า Component ไหนต้อง Render ใหม่ แต่ระบบจะวิ่งตรงไปยัง Text Node หรือ Attribute นั้นๆ แล้วทำการ Update ทันทีด้วยความซับซ้อนระดับ **O(1)**

นี่คือการขจัด Overhead ของ VDOM ทิ้งไปอย่างสมบูรณ์ ทำให้เราได้ Performance ที่ดิบและใกล้เคียงกับการเขียน **Vanilla JS DOM manipulation** ด้วยมือมากที่สุด แต่ยังคงความ Declarative ผ่าน `view!` macro ได้อย่างสวยงาม

---

## 2. Developer Experience (DX) ที่เหนือกว่า

อีกหนึ่งความเจ็บปวดของการเขียน UI ใน **Rust** คือการต่อสู้กับ Borrow Checker เมื่อต้องส่ง State เข้าไปใน Closures หลายๆ ชั้น

**Leptos** แก้ปัญหานี้ด้วยการออกแบบ Primitives ของ Signals ให้เป็น `Copy` และ `'static` สิ่งนี้ไม่ใช่แค่เรื่องของ Syntax Sugar แต่มันคือการออกแบบ Memory Management ที่ชาญฉลาด

> **No More Cloning Hell**
>
> เราสามารถ `move` signal (เช่น `read_signal`, `write_signal`) เข้าไปใน Event Listeners หรือ Derived Signals ได้นับครั้งไม่ถ้วนโดยไม่ต้องสั่ง `.clone()` ให้รก code และไม่ต้องปวดหัวกับ Lifetime hell ซึ่งถือเป็น DX ที่ก้าวกระโดดจาก Framework ยุคก่อนหน้า

---

## 3. Full-stack Isomorphic Framework

แต่ **Leptos** ไม่ได้หยุดแค่การเป็น Client-side Library ที่เร็วเท่านั้น มันถูกวางสถานะเป็น **Full-stack Isomorphic Framework** ตั้งแต่ต้นน้ำ ยิ่งเมื่อทำงานร่วมกับ `cargo-leptos` เราจะเห็นศักยภาพของการทำ **Server Functions** ที่ทำให้เส้นแบ่งระหว่าง Client และ Server จางลง

คุณสามารถเขียนฟังก์ชันดึง Database ในไฟล์เดียวกับ Component แล้วเรียกใช้เหมือนฟังก์ชันปกติ (RPC-like) โดยระบบจะจัดการเรื่อง Serialization/Deserialization ให้เอง

เบื้องหลังมันคือการรองรับมาตรฐาน Web สมัยใหม่อย่างแท้จริง ทั้ง:

- การทำ **HTML Streaming** แบบ Out-of-order
- `<Suspense/>` ที่ช่วยให้ User เห็น Content ได้ไวที่สุดโดยไม่ต้องรอ Data ทั้งก้อน (Holistic Web Performance)

---

## 4. บทสรุป

หากเปรียบเทียบในเชิงปรัชญา ชื่อ **"Leptos"** (ภาษากรีกแปลว่า บาง, เบา, ละเอียด) สะท้อนตัวตนของ Framework ได้ดีที่สุด

- **Yew:** แบก VDOM
- **Dioxus:** โฟกัสที่ Cross-platform (Desktop)
- **Leptos:** เลือกที่จะโฟกัสที่ **"Web Platform"** อย่างเข้มข้นที่สุด โดยใช้ระบบ Reactive ที่ละเอียดอ่อน (Fine-grained) เพื่อตัดส่วนเกินที่ไม่จำเป็นออกไป

> **Recommendation**
>
> สำหรับ Rust Dev ที่กำลังมองหาเครื่องมือที่เคารพทรัพยากรเครื่อง และต้องการเขียน Web App ที่ Scale ได้ด้วย Architecture ที่ถูกต้อง (ไม่ใช่แค่เร็วเพราะเป็น WASM แต่เร็วเพราะ Algorithm การ Update มันถูกออกแบบมาดี) **Leptos** คือคำตอบที่น่าจะตรงใจที่สุดในเวลานี้ครับ

ใครที่สนใจ แนะนำให้ลองดู `cargo-leptos` หรืออ่าน Book ใน Official Docs ดูครับ แล้วคุณจะพบว่า **Rust** บน Web มันไปได้ไกลกว่าที่เราเคยคิดไว้เยอะ ยิ่งล่าสุดพี่สิทธิ์ **Sitt Guruvanich** แชร์เรื่อง **Rust/UI** ก็มาเสริมเติมเต็มได้อย่างดีเลยครับ

---

**Credit & Reference:**

1. [Leptos GitHub repo](https://github.com/leptos-rs/leptos)
2. [Leptos Book](https://github.com/leptos-rs/book)
3. [cargo-leptos GitHub](https://github.com/leptos-rs/cargo-leptos)
