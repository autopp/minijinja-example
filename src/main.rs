use minijinja::Environment;

fn main() {
    // Create a template environment
    let mut env = Environment::new();

    // Add a template
    env.add_template("hello", "Hello {{ name }}!").unwrap();

    // Get the template and render it
    let tmpl = env.get_template("hello").unwrap();
    let result = tmpl
        .render(minijinja::context! { name => "World" })
        .unwrap();

    println!("{}", result);

    // Example with a list
    env.add_template(
        "list",
        "Items:\n{% for item in items %}  - {{ item }}\n{% endfor %}",
    )
    .unwrap();

    let tmpl = env.get_template("list").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            items => vec!["apple", "banana", "cherry"]
        })
        .unwrap();

    println!("\n{}", result);

    // Example with a map
    env.add_template(
        "map",
        "User: {{ user.name }} ({{ user.email }})\nAge: {{ user.age }}",
    )
    .unwrap();

    let tmpl = env.get_template("map").unwrap();
    let result = tmpl
        .render(minijinja::context! {
            user => minijinja::context! {
                name => "Alice",
                email => "alice@example.com",
                age => 30
            }
        })
        .unwrap();

    println!("\n{}", result);
}
