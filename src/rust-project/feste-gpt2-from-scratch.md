# "Feste" เมื่อ Rustacean สร้าง GPT-2 จากศูนย์ ไร้ PyTorch มีแค่ Math และ Memory Layout

> 📅 วันที่เผยแพร่: 2026-02-03

การทำ Machine Learning เรามักคุ้นเคยกับการมอง Tensor เป็นกล่องดำที่ PyTorch จัดการให้ แต่การจะเข้าใจ LLM อย่างถ่องแท้ บางครั้งเราต้องกล้าที่จะทิ้ง abstraction เหล่านั้น แล้วลงมือเขียนมันขึ้นมาเองด้วย #Rust ตั้งแต่บรรทัดแรก (ใครพร้อมก็ตามมาครับ) โปรเจกต์ "Feste" คือตัวอย่างโปรเจกต์ที่จะพาเราเรียนรู้ตั้งแต่การแปลง Text เป็น Byte ไปจนถึงการเขียน Manual Backpropagation เพื่อเทรนโมเดลให้พูดภาษาเชกสเปียร์ได้

มาเริ่มกันที่ Tokenization ปัญหาคลาสสิกของ LLM คือโมเดลไม่ได้อ่าน "ตัวหนังสือ" แบบที่เราอ่าน แต่มันเห็นเป็น token ID การเลือกใช้ Byte Pair Encoding (BPE) แบบ GPT-2 นั้นฉลาดมาก เพราะมันทำงานในระดับ Byte (UTF-8) ทำให้เราจัดการกับ Unicode ภาษาแปลกๆ หรือ Emoji ได้โดยไม่ต้องมี Vocabulary มหาศาล

แต่ความท้าทายทางเทคนิคของ BPE คือขั้นตอนการ Training ที่ต้องนับความถี่ของคู่ Byte ที่ติดกัน (Adjacent pairs) ซ้ำแล้วซ้ำเล่าใน Corpus ขนาดใหญ่ ตรงนี้เองที่ #Rust เฉิดฉาย แทนที่จะรอ #Python วนลูป เราใช้ Rayon เข้ามาจัดการ Parallelism ด้วย `par_chunks` กระจายงานนับคู่ Byte ไปยังทุก Core ของ CPU และใช้ `fold`/`reduce` เพื่อรวมผลลัพธ์จาก `HashMap` ย่อยๆ ทำให้การเทรน Tokenizer ที่ซับซ้อนเสร็จในหลักวินาที โดยที่เรายังคงความถูกต้องของการจัดการ Boundary ระหว่าง chunk ได้อย่างแม่นยำ

เมื่อได้ตัวเลขมาแล้ว เราก็เข้าสู่หัวใจสำคัญนั่นคือ Tensor Operations แทนที่จะใช้ Library สำเร็จรูป เราเลือกสร้าง `struct Tensor` ขึ้นมาเองโดยเก็บข้อมูลดิบเป็น `Vec<f32>` แบบ Flat Array ก้อนเดียว เพื่อให้ Memory Contiguous ที่สุด แล้วใช้ `shape` กับ `strides` เป็นตัวกำหนด View ในการเข้าถึงข้อมูล เทคนิคนี้ทำให้ operations อย่าง `reshape` หรือ `transpose` แทบจะไม่มี cost เพราะเราแค่เปลี่ยนตัวเลข stride โดยไม่ต้องย้ายข้อมูลจริงใน Memory แต่ความมันส์ที่แท้จริงอยู่ที่ Matrix Multiplication ซึ่งเป็นคอขวดของ Deep Learning การเขียน Loop 3 ชั้นแบบ Naive (`for i, j, k`) นั้นฆ่า Performance บน Modern CPU เพราะมันทำลาย Cache Locality อย่างยับเยิน

ทางออกของเราคือการทำ Cache Blocking หรือ Tiling โดยแบ่ง Matrix ออกเป็น Block ย่อยขนาด 8x8 เพื่อให้ข้อมูลฟิตพอดีใน L1 Cache แต่จุดที่ #Rust แสดงพลังขั้นสุดคือตอนที่เรา Optimize Inner Loop ครับ ตอนแรกที่ใช้ Indexing แบบ C-style (`a[i * n + k]`) คอมไพเลอร์ไม่สามารถ Optimize ได้ดีนัก แต่พอเราเปลี่ยนมาใช้ Iterator Pattern (`iter().zip()`) ของ #Rust ตัว LLVM Backend ฉลาดพอที่จะเห็น Pattern การเข้าถึงข้อมูลที่ต่อเนื่อง และทำการ Auto-vectorization สร้างคำสั่ง SIMD (AVX2/NEON) ให้เราอัตโนมัติโดยไม่ต้องเขียน Assembly เอง ผลลัพธ์คือความเร็วที่เพิ่มขึ้นมหาศาลเพียงแค่เขียนโค้ดให้เป็น Idiomatic Rust

เมื่อรากฐานคณิตศาสตร์แน่นปึก เราก็ประกอบร่างเป็น Model Architecture โค้ด #Rust สะท้อนโครงสร้างของ GPT-2 ออกมาอย่างชัดเจน ข้อมูลไหลผ่าน Embedding Layers (ที่บวก Positional Encoding เข้าไปตรงๆ) วิ่งผ่าน Transformer Block 12 ชั้น และจบที่ Layer Norm ตัว Multi-Head Attention ถูก implement โดยการแตก Tensor 768 dimensions ออกเป็น 12 heads (64 dims) แล้วคำนวณ Attention Score พร้อมกัน

สิ่งที่น่าสนใจคือการจัดการ Causal Masking ด้วยเทคนิค `masked_fill` โดยใช้ค่า `-infinity` แทนที่ตำแหน่งในอนาคตก่อนเข้า Softmax เพื่อรับประกันว่าโมเดลจะไม่ "แอบดูเฉลย" (Peeking) ข้อมูลที่ยังมาไม่ถึง นอกจากนี้เรายังใช้ GELU (Gaussian Error Linear Unit) เป็น Activation Function แทน ReLU เพื่อให้ Gradient ไหลได้นุ่มนวลกว่าในช่วงค่าติดลบ ซึ่งสำคัญมากต่อการ Convergence ของโมเดลภาษา

สุดท้ายคือส่วนที่ยากที่สุด Training Infrastructure เราไม่ได้ใช้ Autograd Engine แต่เราเขียน Manual Backpropagation เองทั้งหมดตาม Chain Rule ของ Calculus นี่คือจุดที่ทำให้เราเข้าใจจริงๆ ว่า Gradient ไหลย้อนกลับอย่างไร ตั้งแต่ Loss Function วิ่งย้อนผ่าน Softmax ผ่าน Attention Heads ผ่าน Residual Connections (ที่แค่กระจาย Gradient ไปสองทาง) จนไปถึง Weight Matrices ทุกตัว

จากนั้นเรา Implement AdamW Optimizer เพื่อจัดการ Weight Update โดยแยก Weight Decay ออกจาก Gradient Update ตามสูตรที่ถูกต้อง (ซึ่งต่างจาก Adam ปกติ) และยังมี Gradient Clipping เพื่อป้องกันปัญหา Exploding Gradients ที่มักเกิดกับ Deep Network ทั้งหมดนี้ทำงานอยู่บน Memory Safety Model ของ #Rust ที่ช่วยการันตีว่าเราจะไม่เผลอเขียนทับ Gradient หรือ Access Tensor ผิด Shape ระหว่างการคำนวณที่ซับซ้อนนี้

ผลลัพธ์คือ Feste โมเดลที่เทรนบน CPU เพียวๆ แต่สามารถเรียนรู้ที่จะพูดภาษาเชกสเปียร์ได้จริง กราฟ Loss ลดลงอย่างสวยงาม และ Text Generation เริ่มจับโครงสร้างประโยคและคำศัพท์โบราณได้ การเขียน LLM ด้วย #Rust ครั้งนี้พิสูจน์ให้เห็นว่า เมื่อเราเข้าใจ Low-level operations และใช้เครื่องมือที่ถูกต้อง เราสามารถรีดประสิทธิภาพและควบคุมทุก Byte ของ Memory ได้ดั่งใจ โดยไม่ต้องพึ่งพา Magic ของ Framework ใดๆ

**Credit & Reference:**

1. [feste GitHub repo](https://github.com/tag1consulting/feste)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
