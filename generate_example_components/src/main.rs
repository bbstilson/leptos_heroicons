use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;

use anyhow::Result;
use convert_case::Casing;
use indoc::formatdoc;

fn main() -> Result<()> {
    let file = fs::File::open("../src/lib.rs")?;
    let reader = io::BufReader::new(file);

    let module_re =
        regex::Regex::new(r"(?<indent>\s*)pub mod (?<module_name>\w+) \{")?;
    let component_re = regex::Regex::new(r"pub fn (?<component_name>\w+)\(")?;

    let mut indent = 0;
    let mut module_path_stack: Vec<String> = vec![];
    let mut component_type_to_components: HashMap<String, Vec<String>> =
        HashMap::new();

    for line in reader.lines() {
        let line = line?;

        if let Some(caps) = module_re.captures(&line) {
            let curr_indent = caps["indent"].to_string().len();

            if curr_indent == indent {
                // pop out of the current module and go to the next one
                module_path_stack.pop();
            } else if curr_indent < indent {
                // pop out of the current module
                module_path_stack.pop();
                // pop out of the parent module
                module_path_stack.pop();
            }
            module_path_stack.push(caps["module_name"].to_string());
            indent = curr_indent;
        }

        if let Some(caps) = component_re.captures(&line) {
            let module = module_path_stack.join("::");

            let component_name = caps["component_name"].to_string();
            let component = formatdoc! {r#"
            view! {{ <{component_name} /> }},"#};
            component_type_to_components
                .entry(module)
                .and_modify(|cs| cs.push(component.clone()))
                .or_insert(vec![component]);
        }
    }

    let mut components_file = fs::File::create("../example/src/components.rs")?;
    writeln!(&mut components_file, "use leptos::prelude::*;\n")?;

    for (component_type, components) in component_type_to_components {
        writeln!(
            &mut components_file,
            "{}",
            formatdoc! {r#"
                pub fn {}() -> Vec<impl IntoView> {{
                    use leptos_heroicons::{component_type}::*;

                    vec!["#,
                component_type.replace("::", "_").to_case(convert_case::Case::Snake)
            }
        )?;

        for component in components {
            writeln!(
                &mut components_file,
                "{}",
                format!("        {component}")
            )?;
        }

        writeln!(&mut components_file, "    ]\n}}\n")?;
    }

    Ok(())
}
