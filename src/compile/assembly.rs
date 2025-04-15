use crate::process;

pub fn compile(contents: String) -> String {
    let instructions = process(contents);
    let mut output = String::from(r#"
    section .bss
        mem resb 30000
    section .text
        global _start
    
    _start:
        mov rsi, mem
    "#);

    let mut stack = Vec::new();
    let mut label_id = 0;

    for i in instructions {
        match i {
            '>' => output.push_str("    inc rsi\n"),
            '<' => output.push_str("    dec rsi\n"),
            '+' => output.push_str("    inc byte [rsi]\n"),
            '-' => output.push_str("    dec byte [rsi]\n"),
            '.' => output.push_str(r#"
                mov rax, 1
                mov rdi, 1
                mov rdx, 1
                mov rsi, rsi
                syscall
            "#),
            ',' => output.push_str(r#"
                mov rax, 0
                mov rdi, 0
                mov rdx, 1
                mov rsi, rsi
                syscall
            "#),
            '[' => {
                let id = label_id;
                label_id += 1;

                stack.push(id);
                output.push_str(&format!("loop_start_{}:\n", id));
                output.push_str(&format!(r#"
                    cmp byte [rsi], 0
                    je loop_end_{}
                "#, id));
            },
            ']' => {
                if let Some(id) = stack.pop() {
                    output.push_str(&format!("  cmp byte [rsi], 0\n"));
                    output.push_str(&format!("  jne loop_start_{}\n", id));
                    output.push_str(&format!("loop_end_{}:\n", id));
                }
            },
            _ => {}
        }
    }

    output.push_str(r#"
        mov rax, 60
        xor rdi, rdi
        syscall
    "#);

    output
}
