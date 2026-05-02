# "Redistill" บทพิสูจน์ศักยภาพของ Rust ในการทลายขีดจำกัด Throughput ของ Redis สู่ระดับ 9 ล้าน Ops/sec

> 📅 วันที่เผยแพร่: 2026-02-05

"เราจะสร้าง Key-Value Store ที่เร็วกว่า Redis ได้จริงหรือ?" เป็นคำถามที่ท้าทายมาก แม้ #Redis จะครองบัลลังก์ด้วยความเร็วและความเสถียร แต่นั่นอยู่บนพื้นฐานของสถาปัตยกรรมยุค Memory-first แบบดั้งเดิมที่เป็น Single-threaded ซึ่งกลายเป็นคอขวดสำคัญในยุคของ Modern CPU ที่มี Core มหาศาล ล่าสุดโปรเจกต์ "Redistill"ได้ถือกำเนิดขึ้นด้วยปรัชญาการออกแบบที่น่าสนใจผ่านภาษา #Rust โดยไม่ได้แค่พยายามทำเหมือนแต่เลือกที่จะ "กลั่น" (Distill) เอาเฉพาะแก่นแท้เพื่อรีดประสิทธิภาพสูงสุด จนสามารถทำ Benchmark ชนะทั้ง #Redis และผู้ท้าชิงรุ่นใหม่อย่าง #Dragonfly ได้อย่างขาดลอย

หัวใจสำคัญที่ทำให้ Redistill แตกต่างจาก KV Store ทั่วไป ไม่ใช่แค่การเขียนใหม่ด้วย #Rust แต่คือการตัดสินใจทางสถาปัตยกรรมที่เฉียบคม ผู้พัฒนาได้วิเคราะห์การใช้งานจริงและพบว่า 90% ของคำสั่งที่ใช้งานกันอย่างหนักหน่วงมีเพียงแค่ Strings และ Hashes การตัดส่วนเกินที่ไม่จำเป็นออก ทำให้ Redistill ลด Instruction Footprint และ Optimize Code Path ได้สั้นที่สุด ผนวกกับความสามารถในการจัดการ Memory Safety ของ #Rust ทำให้สามารถสร้างสถาปัตยกรรมแบบ Multi-threaded with Lock-free Reads ได้อย่างสมบูรณ์ ต่างจาก #Redis เดิมที่ติดข้อจำกัดเรื่อง Threading หรือ #Dragonfly (C++) ที่ต้องใช้ Shared-nothing architecture ที่ซับซ้อนกว่าเพื่อเลี่ยง Race condition แต่ #Rust อนุญาตให้ Redistill กระจาย Load ไปยังทุก Core ได้อย่างปลอดภัยและเต็มประสิทธิภาพโดยมี Overhead จากการ Context Switch ที่ต่ำมาก

ผลลัพธ์จากการ Benchmark บนเครื่องระดับ Production-grade อย่าง AWS c7i.16xlarge (Intel 64 cores, 128GB RAM) ด้วย `memtier_benchmark` ได้สร้างตัวเลขที่สั่นสะเทือนวงการ ด้วย Throughput สูงถึง 9.07 ล้าน Operations ต่อวินาที ซึ่งเร็วกว่า #Redis ถึง 4.5 เท่า และที่น่าสนใจกว่านั้นคือเร็วกว่า #Dragonfly (ที่เคลมว่าเป็น Fastest Redis replacement) ถึง 1.7 เท่า แต่สิ่งที่เราให้ความสำคัญยิ่งกว่า Throughput คือ Latency Distribution Redistill ทำค่า p50 Latency ได้ต่ำเพียง 0.479ms (เทียบกับ Redis 2.383ms) และรักษาความเสถียรแม้ที่ p99.9 ไว้ที่ 1.59ms แสดงให้เห็นว่า Rust Runtime และการจัดการ Allocator ภายใน สามารถรับมือกับ Spike Load ได้ดีเยี่ยมโดยไม่มีอาการ GC Pause หรือ Latency Jitter มารบกวน

ในมิติของการใช้งาน Redistill ยังคงรักษาความเข้ากันได้กับ RESP Protocol (Redis Serialization Protocol) ไว้อย่างสมบูรณ์ ซึ่งเป็นกลยุทธ์แบบ Drop-in Replacement ที่ชาญฉลาด เราสามารถใช้ Client Library เดิมอย่าง `ioredis` ใน Node.js หรือ `redis-py` ใน Python เชื่อมต่อเข้ามาได้ทันทีโดยไม่ต้องแก้โค้ดฝั่ง Application ภายใน Redistill ใช้เทคนิคการ Parse ข้อมูลเครือข่ายที่มีประสิทธิภาพสูง (คาดการณ์ว่าเป็น Zero-copy parsing ตามสไตล์ #Rust) ทำให้ได้ Bandwidth สูงถึง 1.58 GB/s รองรับ Use Case ที่ต้องการความเร็วระดับ Extreme เช่น Session Storage, Real-time Leaderboard หรือ API Response Caching ที่ต้องการ Cache Hit Rate สูงและ Eviction Policy ที่แม่นยำ

อย่างไรก็ตาม ในฐานะโปรเจกต์ที่เพิ่งมีอายุเพียง 8 สัปดาห์ และพัฒนาขึ้นเพื่อแก้ Pain Point เฉพาะจุด Redistill จึงมาพร้อมกับ Trade-off ที่ชัดเจน คือการตัดฟีเจอร์ที่ไม่จำเป็นออกเพื่อแลกกับความเร็วสูงสุด ปัจจุบันยังรองรับเพียง Strings และ Hashes เป็นหลัก ยังไม่มี Clustering Mode (เน้น Single-instance Vertical Scaling) และแม้จะมีระบบ Persistence แบบ Snapshot (สำหรับการทำ Warm Restart) แต่โดย Default จะถูกปิดไว้เพื่อให้ Memory Operation ทำงานได้เต็มสูบที่สุด

ผมมองว่า Redistill คือ Use Case Study ชั้นดีที่แสดงให้เห็นว่า #Rust ไม่ได้เป็นเพียงภาษาที่ขายความปลอดภัย (Safety) แต่เป็นเครื่องมือที่ทรงพลังที่สุดในยุคปัจจุบันสำหรับการทำ System Programming ที่ต้องการรีดประสิทธิภาพของ Hardware ออกมาทุกหยด การที่ Developer เพียงคนเดียวสามารถสร้าง Database Engine ที่ Outperform โปรเจกต์ระดับโลกได้ในระยะเวลาอันสั้น คือเครื่องยืนยันว่า Rust Ecosystem และ Concurrency Model คืออนาคตของการสร้าง High-Throughput Systems อย่างแท้จริง

**Credit & Reference:**

1. [Redistill GitHub repo](https://github.com/redistill-io/redistill)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
