pub fn format_eval_output(output: &str) -> String {
    if output.contains("Compiling irust") {
        // Consider this an error
        let mut output_lines: Vec<&str> = output.lines().collect();

        let mut actual_error = false;

        let mut idx = 0;
        while idx < output_lines.len() {
            if output_lines[idx].starts_with("warning") || output_lines[idx].starts_with("error") {
                actual_error = true;
            }

            if output_lines[idx].is_empty() {
                actual_error = false;
            }

            if !actual_error {
                output_lines.remove(idx);
            } else {
                idx += 1;
            }
        }

        output_lines.join("\n")
    } else {
        output.to_owned()
    }
}

pub fn warn_about_common_mistakes(input: &str) -> Option<String> {
    // if input = `x = something`
    if input.split('=').count() == 2 && input.split('=').map(str::trim).all(|s| !s.is_empty()) {
        let warning = "Are you missing a `;` ?\n".to_string();
        return Some(warning);
    }

    // if there were no mistakes return None
    None
}
