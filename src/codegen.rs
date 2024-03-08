use crate::parser::{BinaryOperator, Expression, Program, Statement, UnaryOperator};

pub fn generate_assembly(program: &Program) -> Result<String, String> {
    let mut assembly = String::new();

    for function in &program.functions {
        assembly.push_str(&format!(".globl _{}\n", function.name));
        assembly.push_str(&format!("_{}:\n", function.name));

        for statement in &function.body {
            match statement {
                Statement::Return(expr) => {
                    generate_expression_assembly(expr, &mut assembly)?;
                    assembly.push_str("    ret\n");
                }
            }
        }
        assembly.push_str("\n");
    }

    Ok(assembly)
}

fn generate_expression_assembly(expr: &Expression, assembly: &mut String) -> Result<(), String> {
    match expr {
        Expression::Constant(value) => {
            assembly.push_str(&format!("    mov x0, #{}\n", value));
        }
        Expression::UnaryOp { op, expr } => {
            generate_expression_assembly(expr, assembly)?;
            match op {
                UnaryOperator::Minus => {
                    assembly.push_str("    neg x0, x0\n");
                }
            }
        }
        Expression::BinaryOp { op, left, right } => {
            generate_expression_assembly(right, assembly)?;

            assembly.push_str("    mov x1, x0\n");

            generate_expression_assembly(left, assembly)?;

            match op {
                BinaryOperator::Add => {
                    assembly.push_str("    add x0, x0, x1\n");
                }
                BinaryOperator::Subtract => {
                    assembly.push_str("    sub x0, x0, x1\n");
                }
            }
        }
    }

    Ok(())
}
