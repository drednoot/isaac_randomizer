#include <stdint.h>

enum Error {
  NO_ERR = 0,
  IO_ERR,
  SERIALIZATION_ERR,
  DESERIALIZATION_ERR,
  PARSE_ERR,
  FFI_ERR,
  NULLPTR_ERR,
};

typedef void *Savefile;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int read_savefile(const char *path, Savefile *savefile);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
