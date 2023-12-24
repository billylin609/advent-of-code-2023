#include <stdio.h>
#include <stdlib.h>
#include "day8_exercise3.h"

int main() {
	char file_dir[] = "data/day8_exercise1.txt";
	FILE *file_ptr = f_r_handler_get(file_dir);

	f_get_c(file_ptr);

	return 0;
}
	
char f_get_c(FILE *fptr) {
	char c;

	if (fptr == NULL) {
        	printf("The file is not opened. The program will now exit.");
        	exit(0);
    	} else {
		c = fgetc(fptr);

		printf("%c\n", c);
	}
	return c;
}
	
FILE *f_r_handler_get(char *fdir) {
	FILE *fptr;

	fptr = fopen(fdir, "r");

	return fptr;
}
