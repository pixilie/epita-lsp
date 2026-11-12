use crate::document::*;
use std::collections::HashMap;
use std::sync::Mutex;

use tower_lsp_server::gen_lsp_types::*;
use tower_lsp_server::jsonrpc::Result;
use tower_lsp_server::{Client, LanguageServer};

mod code_action;

#[derive(Debug)]
pub struct Backend {
	pub client: Client,
	pub documents: Mutex<HashMap<Uri, Document>>,
}

impl LanguageServer for Backend {
	async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
		Ok(InitializeResult {
			capabilities: ServerCapabilities {
				code_action_provider: Some(CodeActionProvider::Bool(true)),
				text_document_sync: Some(TextDocumentSync::Kind(TextDocumentSyncKind::Full)),
				execute_command_provider: Some(ExecuteCommandOptions {
					commands: vec!["epita-lsp.count-lines".to_string()],
					..Default::default()
				}),
				..Default::default()
			},
			..Default::default()
		})
	}

	async fn initialized(&self, _: InitializedParams) {
		self.client
			.log_message(MessageType::Info, "epita-lsp: server initialized!")
			.await;
	}

	async fn shutdown(&self) -> Result<()> {
		Ok(())
	}

	async fn did_open(&self, params: DidOpenTextDocumentParams) {
		let doc = Document::new(params.text_document.text, params.text_document.version);
		self.documents
			.lock()
			.unwrap()
			.insert(params.text_document.uri, doc);
	}

	async fn did_change(&self, params: DidChangeTextDocumentParams) {
		let uri = params.text_document.text_document_identifier.uri;
		let version = params.text_document.version;

		let Some(change) = params.content_changes.into_iter().last() else {
			return;
		};

		let new_text = match change {
			TextDocumentContentChangeEvent::TextDocumentContentChangePartial(c) => c.text,
			TextDocumentContentChangeEvent::TextDocumentContentChangeWholeDocument(c) => c.text,
		};

		let mut documents = self.documents.lock().unwrap();
		if let Some(doc) = documents.get_mut(&uri) {
			doc.update(new_text, version);
		}
	}

	async fn did_close(&self, params: DidCloseTextDocumentParams) {
		self.documents
			.lock()
			.unwrap()
			.remove(&params.text_document.uri);
	}

	async fn code_action(
		&self,
		params: CodeActionParams,
	) -> Result<Option<Vec<CodeActionResponse>>> {
		let documents = self.documents.lock().unwrap();
		let Some(doc) = documents.get(&params.text_document.uri) else {
			return Ok(None);
		};

		let actions = code_action::compute(&doc.text, params.range);
		Ok((!actions.is_empty()).then_some(actions))
	}

	async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<LspAny>> {
		if params.command == "epita-lsp.count-lines" {
			let count = &params.arguments.unwrap()[0];
			self.client
				.show_message(MessageType::Info, format!("{count} lines"))
				.await
		}

		Ok(None)
	}
}
