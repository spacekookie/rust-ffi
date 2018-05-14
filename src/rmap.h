
/* Represent the rmap state */
struct rmap_ctx {
    void *map;
};

struct rmap_ctx* rmap_create();

void rmap_insert(struct rmap_ctx **ptr, const char *key, u_int32_t);

u_int32_t rmap_get(struct rmap_ctx **ptr, const char *key);
