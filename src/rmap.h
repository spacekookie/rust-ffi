
/* Represent the rmap state */
struct rmap_ctx {
    void *map;
};

void rmap_create(struct rmap_ctx **ptr);

void rmap_insert(struct rmap_ctx *ptr);

void rmap_test(struct rmap_ctx *ptr);
