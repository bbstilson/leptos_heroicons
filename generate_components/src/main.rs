use anyhow::Result;
use convert_case::Casing;
use indoc::formatdoc;
use std::fs;
use std::io::Write;
use std::path;

mod constants;
mod download_heroicons;

use constants::*;
use download_heroicons::*;

fn main() -> Result<()> {
    download_latest_icons()?;
    extract_latest_icons()?;

    let mut lib_file = fs::File::create(LIB_PATH)?;
    let mut toml_table =
        toml::from_str::<toml::Table>(&fs::read_to_string(TOML_PATH)?)?;

    let mut features = toml::value::Table::from_iter([(
        "default".to_string(),
        toml::Value::Array(vec![]),
    )]);

    generate_library(&mut lib_file, &mut features, OPTIMIZED_PATH, 0)?;

    toml_table.insert("features".to_string(), toml::Value::Table(features));

    fs::write(TOML_PATH, toml::to_string(&toml_table)?)?;

    // remove unzipped icon directory
    fs::remove_dir_all(UNZIPPED_DIR)?;

    Ok(())
}

/// Recursively convert the downloaded icons directory into a library module.
fn generate_library(
    lib_file: &mut fs::File,
    features: &mut toml::map::Map<String, toml::Value>,
    curr: &str,
    depth: usize,
) -> Result<()> {
    let indent = depth * 4;

    for entry_result in fs::read_dir(curr)? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            let dir_name = entry_name(&entry);

            // If we can parse the dir_name as an integer, then we need to add the
            // `size_` prefix.
            let module_name = match dir_name.parse::<i32>() {
                Ok(_) => format!("size_{dir_name}"),
                Err(_) => dir_name,
            };

            let module_start = formatdoc! {r#"
                pub mod {module_name} {{
                    #[allow(unused_imports)]
                    use leptos::*;"#};
            let module_start: String =
                module_start.split('\n').map(indent_line(indent)).collect();

            writeln!(lib_file, "{}", module_start)?;
            generate_library(
                lib_file,
                features,
                &entry.path().to_string_lossy().to_owned(),
                depth + 1,
            )?;
            writeln!(lib_file, "{}\n", format!("{:indent$}}}", ""))?;
        } else {
            // We found an icon. Create a component.
            let icon_name = path::Path::new(&entry_name(&entry))
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let svg = get_and_prep_svg(&entry, indent)?;

            // The feature path is the directory path we took to get to this component
            // it's used to create a specific feature flag (e.g., 24-solid-banana) and
            // also a high-level feature (e.g., 24-solid).
            let feature_path = entry
                .path()
                .parent()
                .unwrap()
                .to_string_lossy()
                .into_owned()
                .strip_prefix(&format!("{}/", OPTIMIZED_PATH))
                .unwrap()
                .replace("/", "-");

            let component =
                make_component(&icon_name, &feature_path, &svg, indent);

            features.insert(
                format!("{feature_path}-{icon_name}"),
                toml::Value::Array(vec![]),
            );
            features.insert(feature_path, toml::Value::Array(vec![]));
            write!(lib_file, "{}", component)?;
        }
    }

    Ok(())
}

fn entry_name(entry: &fs::DirEntry) -> String {
    entry.file_name().to_string_lossy().into_owned()
}

fn get_and_prep_svg(entry: &fs::DirEntry, indent: usize) -> Result<String> {
    let svg = fs::read_to_string(entry.path())?;
    let svg = svg.trim();

    // add the ability to override the classes via props
    let re = regex::Regex::new(r"<svg ").unwrap();
    let svg = re.replace(svg, "<svg class=class ");

    let indented_svg: String =
        svg.split('\n').map(indent_line(indent)).collect();

    Ok(indented_svg.trim().to_string())
}

fn make_component(
    icon_name: &str,
    feature_path: &str,
    svg: &str,
    indent: usize,
) -> String {
    let component_name = icon_name.to_case(convert_case::Case::UpperCamel);
    let feature_name = format!("{feature_path}-{icon_name}");
    let component = formatdoc! {r#"
        #[cfg(any(feature = "{feature_name}", feature = "{feature_path}"))]
        #[component]
        pub fn {component_name}(
            cx: Scope,
            #[prop(optional, into)] class: Option<AttributeValue>,
        ) -> impl IntoView {{
            view! {{ cx,
                {svg}
            }}
        }}
    "#};

    component.split('\n').map(indent_line(indent)).collect()
}

fn indent_line(indent: usize) -> impl FnMut(&str) -> String {
    move |line| format!("{:indent$}{line}\n", "")
}
