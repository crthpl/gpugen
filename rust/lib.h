#include <stdint.h>

typedef struct Generator Generator;

typedef struct Chunk {
  const uint32_t *blocks;
} Chunk;

const struct Generator *new_generator(int32_t min_height, uint32_t height, int32_t sea_level);

void generate_chunk(const struct Generator *generator,
                    void (*callback)(const struct Chunk*),
                    int32_t x,
                    int32_t z);

uint32_t get_height(const struct Generator *generator);

int32_t get_min_height(const struct Generator *generator);

int32_t get_sea_level(const struct Generator *generator);

const char *get_debug_text(const struct Generator *generator, int32_t x, int32_t y, int32_t z);

void free_generator(const struct Generator *generator);

void free_chunk(const struct Chunk *chunk);

void free_string(const char *string);
