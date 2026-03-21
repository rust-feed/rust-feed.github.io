# เมื่อ Rust บุกโลก EVM: เจาะลึก Architecture ของ Arbitrum Stylus ผ่าน Claude Code Skill

> 📅 วันที่เผยแพร่: 2026-02-06

ในการพัฒนา Web3 มักมีช่องว่างขนาดใหญ่ที่น่าอึดอัดใจอยู่เรื่องหนึ่ง ฝั่งหนึ่งเรามี Tutorial ระดับ "Hello World" ที่สอนแค่ Deploy Counter Contract ง่ายๆ แล้วจบไป แต่อีกฝั่งคือ Production Codebase ขนาดมหึมาที่เต็มไปด้วย Configuration ซับซ้อนซึ่งต้องใช้เวลาหลายเดือนในการตกผลึก

ปัญหาที่ Ben Greenberg (DevRel จาก Arbitrum) ค้นพบคือ เมื่อ Developer พยายามใช้ AI อย่าง Claude เพื่อปิดช่องว่างนี้ AI มักจะให้ Code ที่ "ดูเหมือนจะถูก" แต่กลับใช้ SDK เวอร์ชันเก่า หรือใช้ Pattern ที่พังทันทีเมื่อรันบนระบบใหม่อย่าง Arbitrum Stylus

> **Claude Code Skill คืออะไร?**
>
> ชุดความรู้แบบ Structured Markdown ที่ไม่ได้แค่ Gen code แต่ทำหน้าที่เป็น **Engineering Manager** ที่คอยวาง Architecture ให้เราตั้งแต่ต้น และที่น่าสนใจที่สุดคือ Rust ถูกยกให้เป็น **First-class citizen** เคียงคู่กับ Solidity ในฐานะเครื่องมือสำหรับโปรเจกต์ที่ต้องการ **Maximum Performance** และ **Low Gas Cost**

## Rust บน EVM: Memory Model และ sol_storage

หัวใจสำคัญที่ทำให้ Rust เฉิดฉายบน EVM ผ่าน Arbitrum Stylus ได้นั้น อยู่ที่การจัดการความต่างของ Memory Model ระหว่าง Rust และ EVM

> **stylus-sdk: สะพานเชื่อม Rust กับ EVM**
>
> - ใช้ `stylus-sdk` เวอร์ชัน **0.10+** ที่เสถียรต่อ breaking changes
> - Macro `sol_storage!` ทำหน้าที่เป็น **Abstraction Layer** ใน Map Rust Structs → Storage Slots ของ EVM (32-byte key-value pairs)
> - ประกาศ `struct NftContract` พร้อมใช้ Type พิเศษอย่าง `mapping(uint256 => address)` ภายใน Macro ได้เลย

แทนที่เราจะต้องจัดการเรื่อง Slot Hashing เอง เราได้ **Type Safety ตั้งแต่ Compile Time** พร้อมกับประสิทธิภาพระดับ **Native WASM** ซึ่งเป็นสิ่งที่ Solidity แบบดั้งเดิมให้ไม่ได้

## Implementation: ความเข้มงวดที่เป็นเอกลักษณ์ของ Rust

เมื่อเจาะลึกลงไปใน Implementation ของ Contract ผ่าน `impl` block เราจะเห็นความเข้มงวดที่เป็นเอกลักษณ์ของ Rust เข้ามาช่วยกำจัด Bug ระดับ Low-level

> **Rust Patterns ใน Smart Contract**
>
> - `#[entrypoint]` — กำหนดจุดรับ Calldata จากภายนอก
> - `&mut self` — การแก้ไข State ต้องประกาศชัดเจน
> - `.get()` / `.set()` — Stylus บังคับให้ใช้ Pattern นี้สำหรับ Storage field อย่าง `total_supply` หรือ `owners`
> - `U256` / `Address` type จาก `alloy_sol_types` — ป้องกัน Overflow/Underflow

> **ดู Verbose แต่มีเหตุผล**
>
> แม้ Pattern `.get()` / `.set()` จะดูเหมือน Verbose กว่า Solidity เล็กน้อย แต่มันทำให้ Developer **ตระหนักถึง "Cost"** ของการทำ SLOAD และ SSTORE ในทุกบรรทัด — ทุก Storage Access มีค่า Gas!

## Developer Experience: End-to-End Type Safety

ในมุมของ Developer Experience และ Tooling การออกแบบ System Architecture ผ่าน Claude Skill นี้เลือกใช้ Monorepo Structure ที่ขับเคลื่อนด้วย `pnpm workspace` แต่หัวใจหลักยังคงเป็น Cargo ecosystem สำหรับส่วนของ Contract

> **Workflow แบบไร้รอยต่อ**
>
> | ขั้นตอน           | คำสั่ง                    | ผลลัพธ์                      |
> | ----------------- | ------------------------- | ---------------------------- |
> | เขียน Contract    | Rust + `stylus-sdk`       | Type-safe Smart Contract     |
> | Deploy            | `cargo stylus deploy`     | ส่ง WASM ขึ้น Chain          |
> | Generate ABI      | `cargo stylus export-abi` | ABI สำหรับ Frontend          |
> | Frontend เรียกใช้ | `wagmi` + `viem`          | TypeScript catch error ทันที |

โปรเจกต์จะถูก Scaffold ขึ้นมาพร้อมกับ `nitro-devnode` (Local Arbitrum chain ใน Docker) ซึ่งพร้อมรันทันที TypeScript ฝั่ง Frontend จะ Catch error ได้ทันทีถ้าเราเรียกชื่อฟังก์ชันผิดหรือส่ง Type ผิด นี่คือ **End-to-End Type Safety** ที่เชื่อมโลกของ Smart Contract และ Client Side เข้าด้วยกันอย่างสมบูรณ์

## บทสรุป

> **Rust คือ 'Real Deal' ในยุคถัดไปของ Smart Contract**
>
> สิ่งที่ Ben Greenberg นำเสนอไม่ใช่แค่ Tool ช่วยเขียนโค้ด แต่เป็นการประกาศว่า **Rust พร้อมแล้วที่จะเป็นแกนหลักใน Application Layer ของ Blockchain**
>
> Decision Tree ของระบบแนะนำให้เลือก **Stylus Rust** เมื่อต้องการประสิทธิภาพสูงสุด เป็นเครื่องยืนยันว่าเราเขียน Rust บน Blockchain ไม่ใช่เพราะเราชอบภาษานี้ แต่เพราะมันมอบ Architecture ที่เหนือกว่า ทั้งในแง่ของ:
>
> - **Execution Speed**
> - **Gas Optimization**
> - **Memory Safety**

หากใครที่เคยลังเลว่าจะเอาความรู้ Rust มาใช้ในโลก EVM อย่างไร นี่คือ Use case ที่พิสูจน์แล้วว่า Rust คือ "Real Deal" ในยุคถัดไปของ Smart Contract Development

---

**Credit & Reference:**

1. [How I Built a Claude Code Skill That Scaffolds Complete Arbitrum dApps](https://dev.to/arbitrum/how-i-built-a-claude-code-skill-that-scaffolds-complete-arbitrum-dapps-2njl)
2. [arbitrum-dapp-skill Documentation](https://hummusonrails.github.io/arbitrum-dapp-skill/)
3. [X post](https://x.com/hummusonrails/status/2019337368033992833)
4. [Build a dApp on Arbitrum with Claude (Video)](https://www.youtube.com/watch?v=vsejiaOTmJA&t=2s)
5. [arbitrum-dapp-skill GitHub repo](https://github.com/hummusonrails/arbitrum-dapp-skill)
