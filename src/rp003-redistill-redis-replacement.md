# Redistill: บทพิสูจน์ศักยภาพของ Rust ในการทลายขีดจำกัด Throughput ของ Redis สู่ระดับ 9 ล้าน Ops/sec

## 1. บทนำ (Introduction)

"เราจะสร้าง Key-Value Store ที่เร็วกว่า Redis ได้จริงหรือ?" เป็นคำถามที่ท้าทายมาก แม้ **Redis** จะครองบัลลังก์ด้วยความเร็วและความเสถียร แต่นั่นอยู่บนพื้นฐานของสถาปัตยกรรมยุค Memory-first แบบดั้งเดิมที่เป็น Single-threaded ซึ่งกลายเป็นคอขวดสำคัญในยุคของ Modern CPU ที่มี Core มหาศาล

ล่าสุดโปรเจกต์ **Redistill** ได้ถือกำเนิดขึ้นด้วยปรัชญาการออกแบบที่น่าสนใจผ่านภาษา **Rust** โดยไม่ได้แค่พยายามทำเหมือนแต่เลือกที่จะ "กลั่น" (Distill) เอาเฉพาะแก่นแท้เพื่อรีดประสิทธิภาพสูงสุด จนสามารถทำ Benchmark ชนะทั้ง Redis และผู้ท้าชิงรุ่นใหม่อย่าง Dragonfly ได้อย่างขาดลอย

---

## 2. ทำไมต้อง Rust? (Why Rust?)

หัวใจสำคัญที่ทำให้ Redistill แตกต่างจาก KV Store ทั่วไป ไม่ใช่แค่การเขียนใหม่ด้วย Rust แต่คือการตัดสินใจทางสถาปัตยกรรมที่เฉียบคม ผนวกกับความสามารถในการจัดการ Memory Safety ที่ยอดเยี่ยม

> **Safe Concurrency & Lock-free Reads**
>
> การตัดส่วนเกินที่ไม่จำเป็นออก ทำให้ Redistill ลด Instruction Footprint ลงได้มาก ผนวกกับ Ownership Model ของ Rust ทำให้สามารถสร้างสถาปัตยกรรมแบบ **Multi-threaded with Lock-free Reads** ได้อย่างสมบูรณ์แบบ แตกต่างจาก Redis เดิมที่ติดข้อจำกัดกระบวนทัศน์ Single-thread หรือ Dragonfly (C++) ที่ต้องใช้ Shared-nothing architecture ที่ซับซ้อนกว่าเพื่อเลี่ยง Race condition แต่ Rust อนุญาตให้ Redistill กระจาย Load ไปยังทุก Core ได้อย่างปลอดภัยและเต็มประสิทธิภาพโดยมี Overhead จากการ Context Switch ที่ต่ำมาก

---

## 3. สถาปัตยกรรมและการออกแบบ (Architecture & Design)

ผู้พัฒนาได้วิเคราะห์การใช้งานจริงและพบว่า 90% ของคำสั่งที่ใช้งานกันอย่างหนักหน่วงมีเพียงแค่ Strings และ Hashes การตัดส่วนเกินที่ไม่จำเป็นออก ทำให้ Redistill สามารถ Optimize Code Path ได้สั้นที่สุด

ในมิติของการใช้งาน Redistill ยังคงรักษาความเข้ากันได้กับ **RESP Protocol (Redis Serialization Protocol)** ไว้อย่างสมบูรณ์ ซึ่งเป็นกลยุทธ์แบบ Drop-in Replacement ที่ชาญฉลาด เราสามารถใช้ Client Library เดิมอย่าง `ioredis` ใน Node.js หรือ `redis-py` ใน Python เชื่อมต่อเข้ามาได้ทันทีโดยไม่ต้องแก้โค้ดฝั่ง Application

ภายใน Redistill ใช้เทคนิคการ Parse ข้อมูลเครือข่ายที่มีประสิทธิภาพสูง (คาดการณ์ว่าเป็น Zero-copy parsing ตามสไตล์ Rust) ทำให้ได้ Bandwidth สูงถึง **1.58 GB/s** รองรับ Use Case ที่ต้องการความเร็วระดับ Extreme เช่น Session Storage, Real-time Leaderboard หรือ API Response Caching ที่ต้องการ Cache Hit Rate สูงและ Eviction Policy ที่แม่นยำ

---

## 4. Feature Highlights & Performance

ผลลัพธ์จากการ Benchmark บนเครื่องระดับ Production-grade อย่าง AWS c7i.16xlarge (Intel 64 cores, 128GB RAM) ด้วย `memtier_benchmark` ได้สร้างตัวเลขที่สั่นสะเทือนวงการ:

> **Throughput & Latency Distribution**
>
> - **Throughput:** สูงถึง **9.07 ล้าน Operations ต่อวินาที** ซึ่งเร็วกว่า Redis ถึง 4.5 เท่า และเร็วกว่า Dragonfly ถึง 1.7 เท่า
> - **p50 Latency:** ทำได้ต่ำเพียง **0.479ms** (เทียบกับ Redis 2.383ms)
> - **p99.9 Latency:** รักษาความเสถียรไว้ที่ **1.59ms** แสดงให้เห็นว่า Rust Runtime และการจัดการ Allocator ภายใน สามารถรับมือกับ Spike Load ได้ดีเยี่ยมโดยไม่มีอาการ GC Pause หรือ Latency Jitter มารบกวน

---

## 5. บทเรียนและทิศทางของโปรเจกต์ในอนาคต

ในฐานะโปรเจกต์ที่เพิ่งมีอายุเพียง 8 สัปดาห์ และพัฒนาขึ้นเพื่อแก้ Pain Point เฉพาะจุด Redistill จึงมาพร้อมกับ Trade-off ที่ชัดเจน คือ **การตัดฟีเจอร์ที่ไม่จำเป็นออกเพื่อแลกกับความเร็วสูงสุด**

- ปัจจุบันยังรองรับเพียง Strings และ Hashes เป็นหลัก
- ยังไม่มี Clustering Mode (เน้น Single-instance Vertical Scaling)
- แม้จะมีระบบ Persistence แบบ Snapshot (สำหรับการทำ Warm Restart) แต่โดย Default จะถูกปิดไว้เพื่อให้ Memory Operation ทำงานได้เต็มสูบที่สุด

---

## 6. บทสรุป (Conclusion)

ผมมองว่า Redistill คือ Use Case Study ชั้นดีที่แสดงให้เห็นว่า **Rust ไม่ได้เป็นเพียงภาษาที่ขายความปลอดภัย (Safety) แต่เป็นเครื่องมือที่ทรงพลังที่สุดในยุคปัจจุบันสำหรับการทำ System Programming** ที่ต้องการรีดประสิทธิภาพของ Hardware ออกมาทุกหยด

> การที่ Developer เพียงคนเดียวสามารถสร้าง Database Engine ที่ Outperform โปรเจกต์ระดับโลกได้ในระยะเวลาอันสั้น คือเครื่องยืนยันว่า Rust Ecosystem และ Concurrency Model คืออนาคตของการสร้าง High-Throughput Systems อย่างแท้จริง

---

**Credit & Reference:**

1. [Redistill GitHub repo - สำหรับท่านที่สนใจดู Implementation ของ Lock-free data structures หรือวิธีการจัดการ Threading ใน Rust](https://github.com/redistill-io/redistill)
