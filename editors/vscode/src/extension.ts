import { workspace, ExtensionContext } from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
    const serverCommand = process.env.TESTA_LSP_PATH ?? 'testa-lsp';

    const serverOptions: ServerOptions = {
        command: serverCommand,
        args: [],
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'testa' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.testa'),
        },
    };

    client = new LanguageClient(
        'testaLanguageServer',
        'Testa Language Server',
        serverOptions,
        clientOptions
    );

    context.subscriptions.push(client);
    client.start();
}


export function deactivate(): Thenable<void> | undefined {
    return client?.stop();
}
