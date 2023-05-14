// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
const vscode = require('vscode');
const clipboardy = require('node-clipboardy');
const {default: axios} = require('axios');

// This method is called when your extension is activated
// Your extension is activated the very first time the command is executed

/**
 * @param {vscode.ExtensionContext} context
 */
function activate(context) {
	// The command has been defined in the package.json file
	// Now provide the implementation of the command with  registerCommand
	// The commandId parameter must match the command field in package.json

	let disposable = vscode.commands.registerCommand('cortex.explain', function () {

		
		const om = clipboardy.readSync();
		if (typeof om != 'string') {
			vscode.window.showErrorMessage('Cortex: data must be a string');
			return;
		}

		axios.post('https://cortex-vscode-extension.herokuapp.com', {
			original_message: om,
			user_language: "english"
		}).then((result) => {
			vscode.window.showInformationMessage(result.data);
		}).catch((error) => {
			vscode.window.showErrorMessage(error.message);
		});
	});
	context.subscriptions.push(disposable);
}



// This method is called when your extension is deactivated
function deactivate() {}

module.exports = {
	activate,
	deactivate
}

