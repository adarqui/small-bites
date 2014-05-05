#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <readline/readline.h>
#include <readline/history.h>

int main(int argc, char *argv[]) {
	while(1) {
		char * s = readline("prompt> ");
		printf("got: %s\n", s);
		add_history(s);
		free(s);
	}
}
