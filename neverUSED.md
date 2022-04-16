./src/interrupts.rs

```rust
const GICD_ICENABLER: *mut u32 = (GICD_BASE + 0x0180) as *mut u32;
const GICD_ICENABLER_SIZE: u32 = 32;

pub fn disable(interrupt: u32)
```
