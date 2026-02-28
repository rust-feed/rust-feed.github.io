# "Ironpad" เมื่อ Rust กับ AI-Assisted Development พิสูจน์ว่าพวกมันถูกสร้างมาเพื่อกันและกัน

## 1. บทนำ (Introduction)

> (คำเตือน...บทความนี้ยาวมาก 555)

มีนักพัฒนาคนหนึ่งสร้าง full-stack project management application ชื่อ Ironpad ขึ้นมาภายในเวลาเพียง 2 วัน โดยใช้ Rust เป็น backend และพึ่งพา AI-assisted development เกือบทั้งหมด สิ่งที่ทำให้ project นี้น่าสนใจไม่ใช่แค่ความเร็วในการพัฒนา แต่คือการค้นพบว่า Rust และ AI สามารถทำงานร่วมกันได้อย่างลงตัวเหนือความคาดหมาย และที่สำคัญคือผู้พัฒนา document ทุก step ของ development process ไว้แบบ open source ทั้งหมด ทำให้เราได้เห็นภาพที่ชัดเจนว่าการพัฒนาด้วย AI ในยุคนี้เป็นอย่างไร

---

## 2. ทำไมต้อง Rust? ทำไมไม่ใช่ Electron?

ก่อนจะเข้าสู่เนื้อหาเชิงลึกของ development process เรามาดูกันก่อนว่าทำไมผู้พัฒนาถึงเลือก Rust แทนที่จะไปทาง Electron ซึ่งดูจะเป็นทางเลือกที่ง่ายกว่าสำหรับ desktop application Ironpad เป็น local-first, file-based project management system ที่ออกแบบมาให้ files เป็น database โดยตรง ทุก note, task, project เป็น plain Markdown files พร้อม YAML frontmatter ไม่มี cloud, ไม่มี vendor lock-in, ไม่มี proprietary formats คุณสามารถแก้ไขข้อมูลได้ทั้งใน browser UI หรือเปิดไฟล์เดียวกันนั้นใน VS Code, Obsidian, Vim หรือ text editor ใดก็ได้ที่คุณชอบ การเปลี่ยนแปลงจะ sync real-time ผ่าน WebSocket และทุกอย่างถูก version อัตโนมัติด้วย Git

> **Rust vs Electron: ตัวเลขที่พูดแทนทุกอย่าง**
>
> |              | Electron   | Ironpad (Rust) |
> | ------------ | ---------- | -------------- |
> | Bundle Size  | 150-300 MB | **5 MB**       |
> | RAM Usage    | 200-500 MB | **~20 MB**     |
> | Startup Time | 2-5 วินาที | **< 500ms**    |

การเลือก Rust นั้นเป็น deliberate choice ที่มีเหตุผลชัดเจน ความแตกต่างนี้ไม่ใช่แค่ตัวเลข แต่คือ philosophy ที่แตกต่างกันโดยสิ้นเชิง ทุกคนมี browser อยู่แล้ว ทำไมต้อง bundle อีกตัวมาด้วย? Rust backend serve API, Vue frontend รันในบราวเซอร์ที่คุณมีอยู่แล้ว double-click executable มันเปิดบราวเซอร์ให้ คุณก็เริ่มทำงานได้ทันที simple as that

---

## 3. สถาปัตยกรรมของการออกแบบ (Architecture & Design)

สถาปัตยกรรมของ Ironpad ถูกออกแบบมาให้เรียบง่ายตั้งแต่แรก user launch executable จากนั้น Rust backend ที่ build ด้วย Axum 0.8 และ Tokio จะทำงานเป็น REST API server พร้อม WebSocket server สำหรับ real-time sync, file watcher ที่คอยตรวจจับการแก้ไขไฟล์จาก external editors, และ Git auto-commit system ที่ทำงานทุก 60 วินาที frontend คือ Vue 3 พร้อม Vite และ TypeScript ใช้ Milkdown (ซึ่ง based on ProseMirror) เป็น WYSIWYG markdown editor, ใช้ Pinia สำหรับ state management และท้ายสุดข้อมูลทั้งหมดคือ plain Markdown files บน disk ที่คุณสามารถแก้ไขด้วย tool ใดก็ได้

---

### Core Design Decisions

> **4 หลักการออกแบบหลัก**
>
> 1. **Files are the database** — ไม่มี SQLite, ไม่มี IndexedDB, file system คือ source of truth เดียว
> 2. **Backend owns metadata** — IDs, timestamps, frontmatter ถูกจัดการโดย backend อัตโนมัติ
> 3. **External editing is a first-class citizen** — file watcher ตรวจจับการแก้ไขจาก VS Code, Obsidian, Vim แล้ว sync real-time ผ่าน WebSocket
> 4. **Git for everything** — auto-commit ทุก 60 วินาที, manual commit ได้, full diff viewer built-in

---

### API Surface

API surface มีทั้งหมด 29 endpoints ครอบคลุม Notes, Projects, Tasks ทั้งแบบ per-project และ cross-project, Daily notes, Assets, Search, Git operations และ WebSocket endpoint สำหรับ real-time file change notifications สถาปัตยกรรมนี้เรียบง่ายแต่ครอบคลุมทุกสิ่งที่ต้องการสำหรับ productivity application แบบ local-first

---

## 4. Development Process & Tooling: The Open Method

นี่คือจุดที่ project นี้เริ่มน่าสนใจจริงๆ Ironpad ถูกสร้างขึ้นมาด้วย AI-assisted development ทั้งหมด ไม่ใช่แค่ autocomplete แต่คือทุกอย่าง ตั้งแต่ architecture, PRD, implementation, debugging ทั้งหมด ผู้พัฒนาเรียกแนวทางนี้ว่า "Open Method" (ไว้มีโอกาสผมจะมาเล่าให้ฟัง น่าสนใจมากครับในยุคนี้) ไม่ใช่แค่ open source code แต่เป็น open development process ทั้งหมดถูก document ไว้ใน repo ภายใต้ docs/ai-workflow/ ให้ทุกคนได้เรียนรู้

### 6 Phases of AI-Assisted Development

Workflow แบ่งออกเป็น 6 phases ที่แต่ละ phase มีความสำคัญแตกต่างกัน

**Phase 1: Multi-AI Consultation** — ก่อนจะเขียนโค้ดสักบรรทัด ผู้พัฒนาคุยกับ AI models หลายตัว ใช้ Claude สำหรับ architecture, Perplexity สำหรับ library research, Gemini สำหรับ second opinions การใช้เวลา 5 นาทีได้ perspective หลายแบบช่วยประหยัดเวลา rework ได้หลายชั่วโมง

**Phase 2: PRD Creation** — และนี่คือ single highest-leverage activity ของทั้ง project พวกเขาเขียน detailed Product Requirements Document ครอบคลุมทุกอย่าง features, API design, data models, edge cases และที่สำคัญคือระบุชัดเจนว่าอะไร not in scope

> **PRD = 10x ROI**
>
> AI สามารถสร้าง code ที่ดีขึ้นอย่างมากเมื่อมันรู้ชัดเจนว่าความสำเร็จหน้าตาเป็นอย่างไร เวลาที่ใช้กับ PRD จะคืนกลับมาเป็น **10 เท่า**ตอน implementation

**Phase 3: Task Decomposition** — ทำเป็น checklist document แบบง่ายๆ เพื่อทดสอบว่า AI สามารถ handle ได้มากแค่ไหนในครั้งเดียว

**Phase 4: Context Loading** — เนื่องจาก AI models มี training cutoffs ผู้พัฒนาจึงใช้ Context7 (MCP tool) เพื่อ pull current documentation พร้อมกับสร้าง ai-context.md

**Phase 5: Implementation** — build features ใน focused sessions, test, update checklist แล้ว repeat

**Phase 6: Verification** — AI เขียนโค้ด แต่คุณต้อง verify product เอง รันมัน, คลิก buttons, ลอง edge cases

> **อย่าไว้ใจ 'this should work'**
>
> AI จะพูดว่า "this should work" อย่างมั่นใจทุกครั้งโดยไม่มีข้อยกเว้น วิธีแก้คือ **test everything yourself**

### Tools ที่ใช้ใน Development

Tools ที่ใช้ใน development ประกอบด้วย Cursor IDE, Claude Opus, Perplexity AI, Google Gemini, และ Context7

---

## 5. Patterns & Lessons Learned

กลางๆ development มีเหตุการณ์สำคัญเกิดขึ้นที่เปลี่ยน workflow ของ project ไปโดยสิ้นเชิง Claude's context window เพิ่มจาก 200K เป็น 1M tokens และนี่คือการเปลี่ยนแปลงครั้งใหญ่ที่สุดของ project

> **200K vs 1M Tokens: ก่อนและหลัง**
>
> **200K tokens (Claude Opus 4.5):**
>
> - จุได้แค่ 3-5 files พร้อมกัน
> - ต้องแบ่ง features เป็น micro-tasks
> - ต้องทำ handover documents ระหว่างทุก task
> - หา cross-file bugs ได้ยาก
> - Overhead ~15-20 นาที/task สำหรับ context setup
>
> **1M tokens (Claude Opus 4.6):**
>
> - Codebase ทั้งหมด 80+ files ใน context เดียว
> - Implement full features ใน single sessions
> - Cross-file bugs ถูกพบอัตโนมัติ
> - Overhead per task = **zero นาที**

### Codebase Audit: พลังของ 1M Tokens

การ demonstrate ที่ชัดเจนที่สุดของความสามารถนี้คือ codebase audit ผู้พัฒนาโหลด Ironpad codebase ทั้งหมดเข้า single context แล้วถาม "what's wrong?" AI หาเจอ 16 issues รวมถึง bug ที่สำคัญหลายตัว ตัวอย่างแรกคือ **auto-commit silently broken** มี flag หนึ่งที่ไม่เคยถูกเซ็ตเป็น true ที่ไหนเลยในโค้ด การจะหา bug นี้ได้ต้องอ่าน main.rs, git.rs และทุก route handlers พร้อมกัน bug ตัวที่สองคือ **operator precedence bug** ใน JavaScript ที่ `0 > 0` ถูก evaluated ก่อน `??` operator และตัวที่สามคือ **missing atomic writes** มีเพียง 1 ใน 8 write paths ที่ใช้ safe atomic pattern สิ่งที่น่าทึ่งคือ 14 จาก 16 issues ถูกแก้ไขใน single session และ zero compilation errors ถูกสร้างขึ้นระหว่างการแก้ไข การทำ comprehensive audit แบบนี้เป็นไปไม่ได้เลยตอนที่มีแค่ 200K tokens

---

### ทำไม Rust ถึงเหมาะกับ AI-Assisted Development

จากประสบการณ์ตรงของ project นี้ เราได้เห็นภาพที่ชัดเจนว่าทำไม Rust จึงเหมาะกับ AI-assisted development มากเป็นพิเศษ

### Rust's Strict Compiler

> **Rust's Strict Compiler = AI's Best Friend**
>
> > "Rust is excellent for AI-assisted development because the compiler catches entire categories of bugs before runtime"

ด้วยภาษา dynamic languages bugs จะซ่อนตัวจนถึง production แต่ด้วย Rust, `cargo check` คือ mechanical verification pass ที่กำจัด memory safety issues, type mismatches และ missing error handling ได้ในขั้นตอนเดียว compiler ทำหน้าที่เป็น safety net ที่แข็งแกร่งสำหรับโค้ดที่ AI generate ขึ้นมา

### Type System ช่วย AI เข้าใจ Intent

Type system ของ Rust ช่วย AI ในการทำความเข้าใจ intent ด้วย ownership และ borrowing rules AI สามารถ reason เกี่ยวกับ data flow ได้ชัดเจน ด้วย `Result<T, E>` มันเข้าใจ error handling patterns ด้วย async/await มันเห็น async boundaries และด้วย Send/Sync traits มันรู้เรื่อง thread safety ทั้งหมดนี้เป็นข้อมูลที่ embedded ใน type system ไม่ใช่แค่ comments หรือ documentation ที่อาจจะ outdated ได้ compiler บังคับให้ทุกอย่างต้อง correct ตามกฎเหล่านี้

### Ecosystem Maturity

ประการที่สองคือ ecosystem maturity Rust มี crates คุณภาพสูงที่พร้อมใช้งาน project นี้ใช้ Axum 0.8 สำหรับ web framework ซึ่งเป็น ergonomic และ performance สูง, Tokio สำหรับ async runtime ซึ่ง battle-tested และ production-ready, git2 crate สำหรับ Git integration ซึ่งเป็น bindings ของ libgit2 library ที่ใช้กันอย่างแพร่หลาย และ ripgrep สำหรับ search ซึ่งเป็น industry standard tool ที่ให้ผลลัพธ์ sub-100ms การมี crates เหล่านี้หมายความว่า AI ไม่ต้องเขียน low-level code เอง แค่ compose existing components เข้าด้วยกัน และเนื่องจาก crates เหล่านี้มี documentation ดี มี type definitions ชัดเจน AI สามารถใช้งานได้ถูกต้อง

### Error Messages ที่ดีเยี่ยม

ประการที่สามคือ error messages ที่ดีเยี่ยมของ Rust compiler เมื่อ AI generate code ที่ผิด compiler ไม่ได้แค่บอกว่าผิด แต่บอกว่าผิดอย่างไร ทำไมผิด และควรแก้ไขอย่างไร error messages ของ Rust เป็นที่รู้จักว่า helpful มาก และสิ่งนี้ช่วย feedback loop ระหว่าง AI กับ compiler ให้เร็วและมีประสิทธิภาพมากขึ้น AI สามารถอ่าน error message แล้วแก้ไขโค้ดได้ทันทีในหลายกรณี

---

## Patterns ที่ Work และไม่ Work

จากการ document อย่างละเอียด เราได้เห็น patterns ที่ work และไม่ work ชัดเจน

### ✅ สิ่งที่ Work Well

สิ่งที่ work well มีหลายอย่าง ประการแรกซึ่งเคยกล่าวไปแล้วคือ **PRD-first development** นี่คือ highest-leverage activity จริงๆ AI สร้าง code ที่ดีขึ้นอย่างมากเมื่อมันรู้ชัดเจนว่าความสำเร็จหน้าตาเป็นอย่างไร เวลาที่ใช้กับ PRD จะ pay off เป็น 10 เท่าระหว่าง implementation ประการที่สองคือ **Rust's strict compiler** ซึ่งดังที่กล่าวมา compiler catches entire categories of bugs before runtime ทำให้ development loop เร็วและมั่นใจได้มากขึ้น

ประการที่สามคือ **ai-context.md pattern** นี่คือ lean architectural cheat sheet ประมาณ 100 บรรทัดที่บอก AI ว่าควรเขียนโค้ดอย่างไรสำหรับ codebase นี้โดยเฉพาะ โดยไม่มีมัน AI จะคิดค้น patterns ใหม่ๆ เอง โดยมีมัน code จะ match กับ existing conventions อย่างสม่ำเสมอ pattern นี้ช่วยให้ codebase มี consistency แม้ว่าจะถูกเขียนโดย AI หลาย sessions ก็ตาม ประการที่สี่คือ **fresh chats over long conversations** context accumulates noise เมื่อถึง task ที่สามใน chat เดียวกัน AI จะเริ่ม reference irrelevant earlier context การเริ่ม fresh chat พร้อม focused handover สร้างผลลัพธ์ที่ดีกว่าอย่างสม่ำเสมอ

### ❌ สิ่งที่ไม่ Work

> **3 สิ่งที่ต้องระวังเมื่อทำงานกับ AI**
>
> 1. **Trusting "this should work"** — AI จะพูดแบบนี้อย่างมั่นใจทุกครั้ง อย่าเชื่อ ต้อง test everything yourself, click the buttons, try the edge cases
> 2. **Vague requirements** — "Add search" ได้ผลลัพธ์ mediocre แต่ "Add full-text search with ripgrep, triggered by Ctrl+K, showing filename and matching line with context, limited to 5 matches per file" ได้ผลลัพธ์ excellent
> 3. **Over-engineering** — AI ชอบเพิ่ม abstractions ที่ยังไม่ต้องการ บอกตรงๆ ว่า "YAGNI" หรือ "Simplify this" ซึ่ง surprisingly ใช้ได้ผลดี

---

## 6. Feature Highlights

Ironpad มี feature set ที่ครบถ้วนสำหรับ project management application WYSIWYG Markdown editor ที่ใช้ Milkdown ซึ่ง based on ProseMirror พร้อม formatting toolbar, project management สมบูรณ์พร้อม tasks, subtasks, tags, due dates และ recurrence, calendar view ที่มี color-coded task urgency, dashboard ที่แสดง projects ทั้งหมดพร้อม active task summaries, Git integration ครบวงจรตั้งแต่ auto-commit, diff viewer, push/fetch ไปจนถึง conflict detection, real-time sync ที่ทำให้คุณแก้ไขใน VS Code แล้วเห็นการเปลี่ยนแปลงใน browser ทันที, full-text search powered by ripgrep ที่ trigger ด้วย Ctrl+K, dark theme by default และที่สำคัญคือทั้งหมดนี้อยู่ใน binary ขนาด 5 MB ใช้ RAM ประมาณ 20 MB และ startup time sub-second

---

## 7. บทเรียนสำหรับ Community

Ironpad project นี้ให้บทเรียนที่สำคัญหลายอย่างแก่ Rust community ประการแรกคือ **Rust เหมาะกับ AI-assisted development มากกว่าที่เราคิด** compiler's strictness ที่บางครั้งถูกมองว่าเป็นอุปสรรคกลับกลายเป็นข้อได้เปรียบอย่างมากเมื่อทำงานกับ AI type system ช่วย AI reason เกี่ยวกับ code ได้ดีขึ้น error messages ช่วย feedback loop ให้เร็วขึ้น และ compiler catches bugs ที่ AI อาจจะพลาดได้

ประการที่สองคือ **documentation และ process matter มากเท่ากับ code เอง** การที่ Ironpad document ทุก step ของ development process พร้อม PRD, workflow และ lessons learned ทำให้คนอื่นสามารถเรียนรู้และประยุกต์ใช้ได้ นี่คือ "Open Method" ที่ผู้พัฒนาพูดถึง ไม่ใช่แค่ open source code แต่เป็น open development process

ประการที่สามคือ **local-first และ simplicity ยังมีคุณค่าในยุคที่ทุกอย่างเป็น cloud** ไม่ใช่ทุก application ที่ต้องเป็น SaaS ไม่ใช่ทุกข้อมูลที่ต้องไปอยู่บน cloud Ironpad พิสูจน์ว่าคุณสามารถสร้าง powerful productivity tool ที่รันบนเครื่อง local ใช้ resources น้อย แต่ทำงานได้เต็มประสิทธิภาพ

---

## 8. บทสรุป (Conclusion)

> **AI-assisted development is here to stay**
>
> มันทำงานได้ดีกว่าที่หลายคนคิด แต่มันต้องมี process, ต้องมี verification, ต้องมี human judgment ในการตัดสินใจที่สำคัญ AI ไม่ได้แทนที่ developer แต่ **amplify productivity** ของ developer ที่รู้จักใช้มันอย่างถูกวิธี และสำหรับ Rust developers นั้น **compiler คือเพื่อนซี้**ที่ช่วยให้ AI-generated code มี quality สูงขึ้นอย่างเห็นได้ชัด

มีใครเคยลอง AI-assisted development กับ Rust บ้างครับ ประสบการณ์เป็นอย่างไร compiler ช่วยจับ bugs จาก AI-generated code ได้มากน้อยแค่ไหน และ patterns ไหนที่ work well สำหรับการทำงานกับ AI ในโปรเจค Rust ของคุณ ผมว่าถ้าเราแชร์ประสบการณ์กัน community ทั้งหมดจะได้ประโยชน์ครับ

---

**Credit & Reference:**

1. [I Built a Full Project Management App in 2 days Using Claude 4.6](https://dev.to/olaproeis/i-built-a-full-project-management-app-in-2-days-using-claude-47-1e1g)
2. [ironPad GitHub repo](https://github.com/OlaProeis/ironPad)
