use super::cli_interface::CLI;

#[test]
fn hello_world() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo Hello World$").trim(), "Hello World");
}

#[test]
fn add() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {15 + 45}$").trim(), "60");
}

#[test]
fn mul() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {3 * 4}$").trim(), "12");
}

#[test]
fn div() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {21 / 3}$").trim(), "7");
}

#[test]
fn sub() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {21 - 7}$").trim(), "14");
}

#[test]
fn text() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {'Hello World'}$").trim(), "Hello World");
}

#[test]
fn bool() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {true}$").trim(), "1");
    assert_eq!(cli.test_eval("$echo {false}$").trim(), "0");
}

#[test]
fn number() {
    let cli = CLI::new();
    assert_eq!(cli.test_eval("$echo {42}$").trim(), "42");
}

#[test]
fn variable() {
    let cli = CLI::new();
    let code = "
        let x = 42
        $echo {x}$
        x = 21
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "42\n21");
}

#[test]
fn nested_string_interp() {
    let cli = CLI::new();
    let code = "
        let x = 'welcome {'to'} wonderful {'world'}'
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "welcome to wonderful world");
}

#[test]
fn complex_arithmetic() {
    let cli = CLI::new();
    let code = "
        let x = 21
        let y = 2
        let z = 3
        $echo {x + y * z}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "27");
}

#[test]
fn very_complex_arithmetic() {
    let cli = CLI::new();
    let code = "
        let x = 21
        let y = 2
        let z = 6
        let a = 4
        $echo {x + y * z / a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn paranthesis() {
    let cli = CLI::new();
    let code = "
        let x = 21
        let y = 2
        let z = 6
        let a = 2
        $echo {(x + y) * z / a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "69");
}

#[test]
fn command_interpolation() {
    let cli = CLI::new();
    let code = "
        $echo {$echo {$echo Hello World$}$}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "Hello World");
}

#[test]
fn command_inception() {
    let cli = CLI::new();
    let code = "
    ${${${$echo Hello World$}$}$}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "Hello World");
}

#[test]
fn comment() {
    let cli = CLI::new();
    let code = "
        # this is a comment
        let a = 42 # this is a comment as well
    ";
    assert_eq!(cli.test_eval(code).trim(), "");
}

#[test]
fn compare_eq_texts() {
    let cli = CLI::new();
    let code = "
        let x = 'Hello World'
        let y = 'Hello World'
        $echo {x == y}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "1");
}

#[test]
fn compare_eq_numbers() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 42
        $echo {x == y}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "1");
}

#[test]
fn compare_neq_numbers() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        $echo {x != y}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "1");
}

#[test]
fn if_statements() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if x == y {
            $echo {x}$
        } else {
            $echo {y}$
        }
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn if_statements_else() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if x == y {
            $echo {x}$
        }
        else {
            $echo {y}$
        }
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn if_statement_chain() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if {
            x == y {
                $echo {x}$
            }
            else {
                $echo {y}$
            }
        }
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn shorthand_add_text() {
    let cli = CLI::new();
    let code = "
        let x = 'Hello '
        x += 'World'
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "Hello World");
}

#[test]
fn shorthand_add() {
    let cli = CLI::new();
    let code = "
        let x = 16
        x += 48
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "64");
}

#[test]
fn shorthand_sub() {
    let cli = CLI::new();
    let code = "
        let x = 64
        x -= 16
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "48");
}

#[test]
fn shorthand_mul() {
    let cli = CLI::new();
    let code = "
        let x = 16
        x *= 4
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "64");
}

#[test]
fn shorthand_div() {
    let cli = CLI::new();
    let code = "
        let x = 21
        x /= 3
        $echo {x}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "7");
}

#[test]
fn if_statements_singleline() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if x == y => $echo {x}$
        else => $echo {y}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn if_statements_else_singleline() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if x == y => $echo {x}$
        else => $echo {y}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn if_statement_chain_singleline() {
    let cli = CLI::new();
    let code = "
        let x = 42
        let y = 24
        if {
            x == y => $echo {x}$
            else => $echo {y}$
        }
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn ternary_conditional_simple() {
    let cli = CLI::new();
    let code = "
        let a = 12 > 24
            then 42
            else 24
        $echo {a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn ternary_conditional_inline() {
    let cli = CLI::new();
    let code = "
        let a = 12 > 24 then 42 else 24
        $echo {a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn ternary_conditional_nested() {
    let cli = CLI::new();
    let code = "
        let a = 24 > 12
            then (12 > 24
                then 42
                else 24)
            else (12 > 6
                then 24
                else 12)
        $echo {a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "24");
}

#[test]
fn infinite_loop() {
    let cli = CLI::new();
    let code = "
        let a = 0
        loop {
            a += 1
            if a == 5 {
                continue
            }
            $printf \"{a} \"$
            if a == 10 {
                break
            }
        }
    ";
    assert_eq!(cli.test_eval(code).trim(), "1 2 3 4 6 7 8 9 10");
}

#[test]
fn modulo_operator() {
    let cli = CLI::new();
    let code = "
        let a = 10 % 3
        $echo {a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "1");
}

#[test]
fn modulo_shorthand() {
    let cli = CLI::new();
    let code = "
        let a = 10
        a %= 3
        $echo {a}$
    ";
    assert_eq!(cli.test_eval(code).trim(), "1");
}

#[test]
fn function() {
    let cli = CLI::new();
    let code = "
        fun test() {
            $echo Hello World$
        }
        test()
    ";
    assert_eq!(cli.test_eval(code).trim(), "Hello World");
}

#[test]
fn function_with_args() {
    let cli = CLI::new();
    let code = "
        fun test(a, b) {
            $echo {a}$
            $echo {b}$
        }
        test('Hello', 'World')
    ";
    assert_eq!(cli.test_eval(code).trim(), "Hello\nWorld");
}

#[test]
fn function_with_args_different_types() {
    let cli = CLI::new();
    let code = "
        fun test(a, b) {
            $echo {a + b}$
        }
        test('Hello', 'World')
        test(11, 42)
    ";
    assert_eq!(cli.test_eval(code).trim(), "HelloWorld\n53");
}