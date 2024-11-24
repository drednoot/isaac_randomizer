#include <stdint.h>

enum Error {
  NO_ERR = 0,
  IO_ERR,
  SERIALIZATION_ERR,
  DESERIALIZATION_ERR,
  PARSE_ERR,
  FFI_ERR,
  NULLPTR_ERR,
  COULD_NOT_ROLL_ERR,
};

enum Character {
  ISAAC_CHAR = 0,
  MAGDALENE_CHAR,
  CAIN_CHAR,
  JUDAS_CHAR,
  BLUEBABY_CHAR,
  EVE_CHAR,
  SAMSON_CHAR,
  AZAZEL_CHAR,
  LAZARUS_CHAR,
  EDEN_CHAR,
  LOST_CHAR,
  LILITH_CHAR,
  KEEPER_CHAR,
  APOLLYON_CHAR,
  FORGOTTEN_CHAR,
  BETHANY_CHAR,
  JACOBANDESAU_CHAR,
  TAINTEDISAAC_CHAR,
  TAINTEDMAGDALENE_CHAR,
  TAINTEDCAIN_CHAR,
  TAINTEDJUDAS_CHAR,
  TAINTEDBLUEBABY_CHAR,
  TAINTEDEVE_CHAR,
  TAINTEDSAMSON_CHAR,
  TAINTEDAZAZEL_CHAR,
  TAINTEDLAZARUS_CHAR,
  TAINTEDEDEN_CHAR,
  TAINTEDLOST_CHAR,
  TAINTEDLILITH_CHAR,
  TAINTEDKEEPER_CHAR,
  TAINTEDAPOLLYON_CHAR,
  TAINTEDFORGOTTEN_CHAR,
  TAINTEDBETHANY_CHAR,
  TAINTEDJACOB_CHAR,
};

enum Target {
  BLUEBABY_TARG = 0,
  LAMB_TARG,
  MEGASATAN_TARG,
  DELIRIUM_TARG,
  BEAST_TARG,
  MOTHER_TARG,
  ULTRAGREED_TARG,
  BOSSRUSH_TARG,
  HUSH_TARG,

  SATAN_TARG,
  ISAAC_TARG,
  HEART_TARG,

  MOM_TARG,
  TARGET_COUNT,
};

typedef void *UnlocksHandle;
typedef char *RustString;

typedef struct {
  enum Character character;
  int targets;
} RunTarget;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int read_unlocks_from_file(const char *path, UnlocksHandle *savefile_out);
int free_unlocks(const UnlocksHandle unlocks_handle);

int print_character(enum Character character, RustString *str);
int print_target(enum Target target, RustString *str);
int free_string(RustString str);

int randomize(const UnlocksHandle unlocks_handle, RunTarget *targets_out);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
