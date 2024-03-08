use crate::parser::{Expression, Program, Statement};

pub fn generate_assembly(program: &Program) -> Result<String, String> {
    let mut assembly = String::new();

    for function in &program.functions {
        assembly.push_str(&format!(".globl _{}\n", function.name));
        assembly.push_str(&format!("_{}:\n", function.name));

        for statement in &function.body {
            match statement {
                Statement::Return(Expression::Constant(value)) => {
                    assembly.push_str(&format!("    mov x0, #{}\n", value));
                    assembly.push_str("    ret\n");
                }
            }
        }
        assembly.push_str("\n");
    }

    Ok(assembly)
}
