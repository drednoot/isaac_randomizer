#include <stdio.h>

#include "isaac.h"

int main(void) {
  UnlocksHandle unlocks;

  read_unlocks_from_file("/home/ns/Downloads/sf.toml", &unlocks);

  RunTarget target;
  randomize(unlocks, &target);

  RustString character;
  print_character(target.character, &character);

  printf("%s\n\nVS\n\n", character);

  for (int i = 0; i < TARGET_COUNT; ++i) {
    if (target.targets & (1 << i)) {
      RustString target;
      print_target((enum Target)i, &target);
      printf("%s\n", target);
      free_string(target);
    }
  }

  free_string(character);
  free_unlocks(unlocks);
  return 0;
}
