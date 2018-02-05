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

    struct rmap_ctx *ctx;
    rmap_create(&ctx);

    printf("rmap address: %d\n", (int) ctx);

    printf("About to insert...\n");
    rmap_insert(ctx);

    rmap_test(ctx);
    return 0;
}