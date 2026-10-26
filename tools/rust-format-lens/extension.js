const vscode = require("vscode");

function activate(context) {
  const changes = new vscode.EventEmitter();
  context.subscriptions.push(
    changes,
    vscode.languages.onDidChangeDiagnostics(() => changes.fire()),
    vscode.commands.registerCommand("rustFormatLens.format", async (uri) => {
      const document = uri
        ? await vscode.workspace.openTextDocument(uri)
        : vscode.window.activeTextEditor?.document;
      if (!document || document.languageId !== "rust") return;
      await vscode.window.showTextDocument(document);
      await vscode.commands.executeCommand("editor.action.formatDocument");
    }),
    vscode.languages.registerCodeLensProvider(
      { language: "rust", scheme: "file" },
      {
        onDidChangeCodeLenses: changes.event,
        async provideCodeLenses(document, token) {
          const symbols = await vscode.commands.executeCommand(
            "vscode.executeDocumentSymbolProvider",
            document.uri,
          );
          if (token.isCancellationRequested) return [];
          return (symbols ?? [])
            .filter(
              (symbol) =>
                symbol.name === "main" &&
                symbol.kind === vscode.SymbolKind.Function,
            )
            .map(
              (symbol) =>
                new vscode.CodeLens(
                  new vscode.Range(
                    (symbol.selectionRange ?? symbol.location.range).start,
                    (symbol.selectionRange ?? symbol.location.range).start,
                  ),
                  {
                    title: "$(code) Format",
                    command: "rustFormatLens.format",
                    arguments: [document.uri],
                  },
                ),
            );
        },
      },
    ),
  );
}

module.exports = { activate };
