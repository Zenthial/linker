#include <stdio.h>
#include <string.h>
#include <strings.h>

int main(void) {
  const char *str = "yo";

  if (strnlen(str, 3) != 2) {
    fprintf(stderr, "error");
  } else {
    printf("works");
  }

  return 0;
}
