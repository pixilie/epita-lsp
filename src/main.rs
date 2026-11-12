use epita_lsp::backend::Backend;
use std::collections::HashMap;
use std::sync::Mutex;
use tower_lsp_server::{LspService, Server};

#[tokio::main]
async fn main() {
	let stdin = tokio::io::stdin();
	let stdout = tokio::io::stdout();

	let (service, socket) = LspService::new(|client| Backend {
		client,
		documents: Mutex::new(HashMap::new()),
	});
	Server::new(stdin, stdout, socket).serve(service).await;
}
