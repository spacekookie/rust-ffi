/*
 * This code is part of the qaul.net Rust experimentation
 * effort.
 */

#include <stdlib.h>
#include <stdio.h>
#include "rmap.h"

int main(int argn, char **argv)
{
    printf("Welcome to the   R U S T   M A P   T E S T\n");

    struct rmap_ctx *ctx = rmap_create();
    printf("rmap address: %d\n", (int) ctx);

    printf("About to insert...\n");
    rmap_insert(&ctx, "Some key", 42);

    printf("Value is: %d", rmap_get(&ctx, "Some key"));
    return 0;
}