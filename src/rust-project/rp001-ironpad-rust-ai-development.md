# "Ironpad" เมื่อ Rust กับ AI-Assisted Development พิสูจน์ว่าพวกมันถูกสร้างมาเพื่อกันและกัน

> 📅 วันที่เผยแพร่: 2026-02-08

(คำเตือน...บทความนี้ยาวมาก 555) มีนักพัฒนาคนหนึ่งสร้าง full-stack project management application ชื่อ Ironpad ขึ้นมาภายในเวลาเพียง 2 วัน โดยใช้ Rust เป็น backend และพึ่งพา AI-assisted development เกือบทั้งหมด สิ่งที่ทำให้ project นี้น่าสนใจไม่ใช่แค่ความเร็วในการพัฒนา แต่คือการค้นพบว่า Rust และ AI สามารถทำงานร่วมกันได้อย่างลงตัวเหนือความคาดหมาย และที่สำคัญคือผู้พัฒนา document ทุก step ของ development process ไว้แบบ open source ทั้งหมด ทำให้เราได้เห็นภาพที่ชัดเจนว่าการพัฒนาด้วย AI ในยุคนี้เป็นอย่างไร

## ทำไมถึงเลือก Rust สำหรับ Local-first App

ก่อนจะเข้าสู่เนื้อหาเชิงลึกของ development process เรามาดูกันก่อนว่าทำไมผู้พัฒนาถึงเลือก Rust แทนที่จะไปทาง Electron ซึ่งดูจะเป็นทางเลือกที่ง่ายกว่าสำหรับ desktop application Ironpad เป็น local-first, file-based project management system ที่ออกแบบมาให้ files เป็น database โดยตรง ทุก note, task, project เป็น plain Markdown files พร้อม YAML frontmatter ไม่มี cloud, ไม่มี vendor lock-in, ไม่มี proprietary formats คุณสามารถแก้ไขข้อมูลได้ทั้งใน browser UI หรือเปิดไฟล์เดียวกันนั้นใน VS Code, Obsidian, Vim หรือ text editor ใดก็ได้ที่คุณชอบ การเปลี่ยนแปลงจะ sync real-time ผ่าน WebSocket และทุกอย่างถูก version อัตโนมัติด้วย Git

การเลือก Rust นั้นเป็น deliberate choice ที่มีเหตุผลชัดเจน เมื่อเทียบกับ Electron app ที่มี bundle size 150-300 MB, ใช้ RAM 200-500 MB และใช้เวลา startup 2-5 วินาที Ironpad ที่เขียนด้วย Rust กลับได้ binary ขนาดเพียง 5 MB, ใช้ RAM แค่ 20 MB และ startup ใน sub-second (ต่ำกว่า 500ms) ความแตกต่างนี้ไม่ใช่แค่ตัวเลข แต่คือ philosophy ที่แตกต่างกันโดยสิ้นเชิง ทุกคนมี browser อยู่แล้ว ทำไมต้อง bundle อีกตัวมาด้วย? Rust backend serve API, Vue frontend รันในบราวเซอร์ที่คุณมีอยู่แล้ว double-click executable มันเปิดบราวเซอร์ให้ คุณก็เริ่มทำงานได้ทันที simple as that

## สถาปัตยกรรมและ Core Design Decisions

สถาปัตยกรรมของ Ironpad ถูกออกแบบมาให้เรียบง่ายตั้งแต่แรก user launch executable จากนั้น Rust backend ที่ build ด้วย Axum 0.8 และ Tokio จะทำงานเป็น REST API server พร้อม WebSocket server สำหรับ real-time sync, file watcher ที่คอยตรวจจับการแก้ไขไฟล์จาก external editors, และ Git auto-commit system ที่ทำงานทุก 60 วินาที frontend คือ Vue 3 พร้อม Vite และ TypeScript ใช้ Milkdown (ซึ่ง based on ProseMirror) เป็น WYSIWYG markdown editor, ใช้ Pinia สำหรับ state management และท้ายสุดข้อมูลทั้งหมดคือ plain Markdown files บน disk ที่คุณสามารถแก้ไขด้วย tool ใดก็ได้

Core design decisions มีหลายจุดที่น่าสนใจมาก
ประการแรกคือ files are the database ไม่มี SQLite, ไม่มี IndexedDB, file system คือ source of truth เดียว
ประการที่สองคือ backend owns metadata ซึ่งหมายความว่า IDs, timestamps, และ frontmatter ถูกจัดการโดย backend อัตโนมัติ users ไม่ต้องแก้ไข metadata ด้วยตัวเอง
ประการที่สามซึ่งสำคัญมากคือ external editing is a first-class citizen file watcher จะตรวจจับการเปลี่ยนแปลงจาก VS Code, Obsidian, Vim หรือ editor อื่นๆ และ sync ไปยัง browser UI แบบ real-time ผ่าน WebSocket
และสุดท้ายคือ Git for everything auto-commit ทุก 60 วินาที, manual commit พร้อม custom messages ได้, และมี full diff viewer built-in ใน UI

API surface มีทั้งหมด 29 endpoints ครอบคลุม Notes, Projects, Tasks ทั้งแบบ per-project และ cross-project, Daily notes, Assets (upload และ serve), Search, Git operations และ WebSocket endpoint สำหรับ real-time file change notifications สถาปัตยกรรมนี้เรียบง่ายแต่ครอบคลุมทุกสิ่งที่ต้องการสำหรับ productivity application แบบ local-first

## AI-Assisted Development ด้วย "Open Method"

นี่คือจุดที่ project นี้เริ่มน่าสนใจจริงๆ Ironpad ถูกสร้างขึ้นมาด้วย AI-assisted development ทั้งหมด ไม่ใช่แค่ autocomplete แต่คือทุกอย่าง ตั้งแต่ architecture, PRD, implementation, debugging ทั้งหมด ผู้พัฒนาเรียกแนวทางนี้ว่า "Open Method" (ไว้มีโอกาสผมจะมาเล่าให้ฟัง น่าสนใจมากครับในยุคนี้) ไม่ใช่แค่ open source code แต่เป็น open development process ทั้งหมดถูก document ไว้ใน repo ภายใต้ `docs/ai-workflow/` ให้ทุกคนได้เรียนรู้

Workflow แบ่งออกเป็น 6 phases ที่แต่ละ phase มีความสำคัญแตกต่างกัน

- **Phase 1 คือ Multi-AI Consultation** ก่อนจะเขียนโค้ดสักบรรทัด ผู้พัฒนาคุยกับ AI models หลายตัว ใช้ Claude สำหรับ architecture, Perplexity สำหรับ library research, Gemini สำหรับ second opinions
- **Phase 2 คือ PRD Creation** และนี่คือ single highest-leverage activity ของทั้ง project พวกเขาเขียน detailed Product Requirements Document ครอบคลุมทุกอย่าง features, API design, data models, edge cases
- **Phase 3 คือ Task Decomposition** แบบง่ายๆ เพื่อทดสอบว่า AI สามารถ handle ได้มากแค่ไหนในครั้งเดียว
- **Phase 4 คือ Context Loading** ผู้พัฒนาจึงใช้ Context7 (MCP tool) เพื่อ pull current documentation พร้อมกับสร้าง `ai-context.md`
- **Phase 5 คือ Implementation** ที่ build features ใน focused sessions, test, update checklist แล้ว repeat
- **Phase 6 คือ Verification** ซึ่งเป็นหลักการสำคัญ AI เขียนโค้ด แต่คุณต้อง verify product เอง รันมัน, ลอง edge cases

Tools ที่ใช้ใน development ประกอบด้วย Cursor IDE, Claude Opus 4.5/4.6, Perplexity AI, Google Gemini และ Context7 (MCP)

## ความเปลี่ยนแปลงเมื่อ AI Context ขยายเป็น 1M Tokens

กลางๆ development มีเหตุการณ์สำคัญเกิดขึ้นที่เปลี่ยน workflow ของ project ไปโดยสิ้นเชิง Claude's context window เพิ่มจาก 200K เป็น 1M tokens การใช้ Claude 4.5 200K ต้องแบ่ง features ยิบย่อย แต่เมื่อเปลี่ยนมาใช้ Claude Opus 4.6 กับ 1M tokens ทุกอย่างเปลี่ยนไป codebase ทั้งหมด 80+ files พอดีใน context เดียว สามารถ implement full features ใน single sessions ได้เลย

การ demonstrate ที่ชัดเจนที่สุดคือ codebase audit ผู้พัฒนาโหลด Ironpad codebase ทั้งหมดแล้วถาม "what's wrong?" AI หาเจอ 16 issues รวมถึง bug ที่สำคัญหลายตัว 14 จาก 16 issues ถูกแก้ไขใน single session การทำ comprehensive audit แบบนี้เป็นไปไม่ได้เลยตอนที่มีแค่ 200K tokens

## ทำไม Rust ถึงคู่ควรกับ AI Development?

จากประสบการณ์ตรงของ project นี้ เราได้เห็นภาพที่ชัดเจนว่าทำไม Rust จึงเหมาะกับ AI-assisted development มากเป็นพิเศษ

1. **Rust's strict compiler**: ผู้พัฒนาระบุชัดเจนว่า "Rust is excellent for AI-assisted development because the compiler catches entire categories of bugs before runtime" compiler ทำหน้าที่เป็น safety net ที่แข็งแกร่งสำหรับโค้ดที่ AI generate ขึ้นมา
2. **Type system ของ Rust**: ช่วย AI ในการทำความเข้าใจ intent ด้วย ownership และ borrowing rules AI สามารถ reason เกี่ยวกับ data flow ได้ชัดเจน
3. **Ecosystem maturity**: Rust มี crates คุณภาพสูงที่พร้อมใช้งาน project นี้ใช้ Axum 0.8, Tokio, git2, ripgrep การมี crates เหล่านี้หมายความว่า AI ไม่ต้องเขียน low-level code เอง
4. **Error messages ที่ดีเยี่ยม**: เมื่อ AI generate code ที่ผิด compiler ไม่ได้แค่บอกว่าผิด แต่บอกว่าผิดอย่างไร ทำไมผิด และควรแก้ไขอย่างไร ช่วย Feedback loop ให้การแก้ไขไวขึ้น

## บทเรียนจาก Project: Patterns for AI Coding

จากการ document อย่างละเอียด เราได้เห็น patterns ที่ work และไม่ work ชัดเจน

**สิ่งที่ work well**: PRD-first development คือ highest-leverage activity การใช้ `ai-context.md` pattern เพื่อเก็บ architecture rules สำหรับ codebase การเริ่ม fresh chats over long conversations เพื่อป้องกัน context noises

**สิ่งที่ไม่ work**: การ trusting "this should work" จาก AI วิธีแก้คือต้อง Test everything การใช้ requirements กว้างๆ (เช่น "Add search" สู้ระบุเจาะจงไม่ได้) และเรื่อง over-engineering ที่ AI มักเพิ่มสิ่งที่เผื่ออนาคตวิธีแก้คือต้องคอยย้ำ YAGNI (You Aren't Gonna Need It) เสมอ

## บทสรุปของ Ironpad

Ironpad มี feature set ที่ครบถ้วนสำหรับ project management application เครื่องมือสุดแรงเร็วในขนาด binary 5 MB ใช้ RAM 20 MB และ startup time sub-second บทเรียนสำคัญคือ AI-assisted development is here to stay มันต้องกระทำควบคู่กับ process, verification และ human judgment AI เป็นตัวขยายพลังของนักพัฒนา โดยเฉพาะฝั่ง Rust ที่ตัว Compiler เป็นเพื่อนซี้ช่วยคัดกรองคุณภาพ Code

สำหรับใครที่สนใจสามารถลองเล่น Ironpad ได้ที่ GitHub repo เลยครับ แล้วชุมชนเรามีใครใช้ AI Assisted ในการพัฒนา Rust บ้างไหม แชร์คุยกันได้เลย!

---

**Credit & Reference:**

1. [I Built a Full Project Management App in 2 days Using Claude 4.6](https://dev.to/olaproeis/i-built-a-full-project-management-app-in-2-days-using-claude-47-1e1g)
2. [ironPad GitHub repo](https://github.com/OlaProeis/ironPad)

<!-- NAVIGATION:START -->
<!-- markdownlint-disable MD033 -->
<div class="article-nav">
  <a class="nav-left" href="../index.html">← Introduction</a>
  <a class="nav-right" href="./index.html">Category index →</a>
</div>
<!-- markdownlint-enable MD033 -->
<!-- NAVIGATION:END -->
