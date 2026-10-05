fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap_or_else(|| "lab1-ai-agent".to_string());

    let name = match args.next() {
        Some(name) => name,
        None => {
            eprintln!("Usage: {program} <name>");
            std::process::exit(1);
        }
    };

    println!("{}", greet(&name));
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn greeting_contains_name() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }

    #[test]
    fn greeting_works_for_other_names() {
        assert_eq!(greet("Bob"), "Hello, Bob!");
    }
}
