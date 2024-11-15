use std::arch::asm;

#[allow(asm_sub_register)]
fn add(a: i32, b: i32) -> i32 {
    let result: i32;
    unsafe {
        asm!(
        "add {0}, {1}, {2}",     // Inline assembly code
        out(reg) result,         // Output operand
        in(reg) a,               // Input operand
        in(reg) b                // Input operand
        );
    }
    result
}

fn main() {
    let x = 5;
    let y = 3;
    let sum = add(x, y);
    println!("Sum: {}", sum); // Output: Sum: 8
}
