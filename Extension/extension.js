// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = require('vscode');
const clipboardy = require('node-clipboardy');
const {default: axios} = require('axios');


// https://cortex-vscode-extension.herokuapp.com

async function sendRequest(instruction, content) {
	try {
		const res = await axios.post("http://localhost:3000", {
			original_message: content,
			instruction: instruction,
			user_language: "english"
		});
		vscode.window.showInformationMessage(res.data);
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

	let exp_disposable = vscode.commands.registerCommand('cortex.explain', async function () {

		
		// const content = clipboardy.readSync();

		// if (typeof content != 'string') {
		// 	vscode.window.showErrorMessage('Cortex: data must be a string');
		// 	return;
		// }

		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection);

		await sendRequest("Explain", content);
	});

	let opt_disposable = vscode.commands.registerCommand('cortex.optimize', function () {

		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection);

		sendRequest("Optimize", content);
	});

	let deb_disposable = vscode.commands.registerCommand('cortex.debug', function () {

		const editor = vscode.window.activeTextEditor;
		const content = editor.document.getText(editor.selection);

		sendRequest("Debug", content);

	});


	context.subscriptions.push( exp_disposable, opt_disposable, deb_disposable);
}



// This method is called when your extension is deactivated
function deactivate() {}

module.exports = {
	activate,
	deactivate
}
