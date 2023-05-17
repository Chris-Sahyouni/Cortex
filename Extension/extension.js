// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = require('vscode');
const {default: axios} = require('axios');

async function sendRequest(instruction, content, outChannel) {
	try {
		const res = await axios.post(process.env.SERVER_URL, {
			original_message: content,
			instruction: instruction,
			user_language: "english"
		});
		outChannel.show(true);
		outChannel.appendLine("----------------------------------------------------------------------");
		outChannel.appendLine(res.data);
	} catch (error) {
		vscode.window.showErrorMessage(error.message);
	}
}

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
	// The command has been defined in the package.json file
	// Now provide the implementation of the command with  registerCommand
	// The commandId parameter must match the command field 4in package.json

	const outChannel = vscode.window.createOutputChannel('Cortex', "*");
	

	let exp_disposable = vscode.commands.registerCommand('cortex.explain', function () {
		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection)
		outChannel.show(true);
		sendRequest("Explain", content, outChannel);
	});

	let opt_disposable = vscode.commands.registerCommand('cortex.optimize', function () {
		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection);
		outChannel.show(true);
		sendRequest("Optimize", content, outChannel);
	});

	let deb_disposable = vscode.commands.registerCommand('cortex.debug', function () {
		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection);
		outChannel.show(true);
		sendRequest("Debug", content, outChannel);
	});


	context.subscriptions.push(exp_disposable, opt_disposable, deb_disposable);
}



// This method is called when your extension is deactivated
function deactivate() {}

module.exports = {
	activate,
	deactivate
}
