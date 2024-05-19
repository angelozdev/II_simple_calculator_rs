use std::io::stdin;

fn get_input() -> Result<String, String> {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .map_err(|_| "No se pudo obtener información")?;

    let input = input.trim().to_string();

    if input.is_empty() {
        return Err(String::from("Debes ingresar una expresión válida."));
    }

    Ok(input)
}

fn parse_expression(input: &str) -> Result<(f64, char, f64), String> {
    let arr: Vec<&str> = input.split_whitespace().collect();

    if arr.len() != 3 {
        return Err(format!(
            "Se esperaban tres parámetros, pero se encontraron {}.",
            arr.len(),
        ));
    }

    let operand1: f64 = arr[0]
        .parse()
        .map_err(|_| format!("'{}' no es un número válido.", arr[0]))?;

    let operator: char = arr[1]
        .parse()
        .map_err(|_| format!("{} no es un operador válido.", arr[1]))?;

    let operand2: f64 = arr[2]
        .parse()
        .map_err(|_| format!("'{}' no es un número válido", arr[2]))?;

    Ok((operand1, operator, operand2))
}

fn execute_operation(n: f64, op: char, m: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(n + m),
        '-' => Ok(n - m),
        '/' => {
            if m == 0.0 {
                return Err(format!("'{}' no se puede dividir entre 0.", n));
            }

            Ok(n / m)
        }
        '*' => Ok(n * m),
        '%' => Ok(n % m),
        op => Err(format!("'{}' no es un operador válido.", op)),
    }
}

fn main() {
    loop {
        println!("Ingresa una expresion aritmética (e.g., 3 + 4): ");
        let input = get_input();

        match parse_expression(&input.unwrap()) {
            Ok((n, op, m)) => match execute_operation(n, op, m) {
                Ok(result) => println!("{}", result),
                Err(err) => eprintln!("{}", err),
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}
