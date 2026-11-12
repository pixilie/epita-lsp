use tower_lsp_server::gen_lsp_types::*;

pub(super) fn compute(content: &str, range: Range) -> Vec<CodeActionResponse> {
	let mut actions = Vec::new();

	if let Some(action) = count_lines_action(content, range) {
		actions.push(action);
	}

	actions
}

fn count_lines_action(content: &str, range: Range) -> Option<CodeActionResponse> {
	if range.end.line - range.start.line < 10 {
		return None;
	}

	let lines = count_lines(range.start.line as usize, range.end.line as usize, content);

	Some(CodeActionResponse::Command(Command {
		title: "Count lines".to_string(),
		tooltip: Some("".to_string()),
		command: "epita-lsp.count-lines".to_string(),
		arguments: Some(vec![lines.into()]),
	}))
}

pub fn count_lines(start: usize, end: usize, text: &str) -> usize {
	text.lines()
		.skip(start)
		.take(end - start)
		.filter(|line| {
			let t = line.trim();
			!(t.is_empty() || t == "{" || t == "}" || t.starts_with("//"))
		})
		.count()
}
