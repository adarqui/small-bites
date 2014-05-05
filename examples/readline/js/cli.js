var readline = require('readline');

var rl = readline.createInterface({
	input : process.stdin,
	output : process.stdout
});

rl.setPrompt('prompt> ');
rl.prompt();

rl.on('SIGINT', function() {
	console.log("INT");
	process.exit(0);
});

rl.on('line', function(line) {
	console.log(line);
//	rl.write(line);
	rl.prompt();
});

rl.on('close', function() {
	console.log('closing');
	process.exit(0);
});
