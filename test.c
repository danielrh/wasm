#include <stdio.h>
extern unsigned char *wasm_alloc_u8(unsigned long);
extern void wasm_free_u8(unsigned char *,unsigned long);

int main() {
    int i;
    unsigned char *ptr = wasm_alloc_u8(16);
    printf("%d\n", wasm_checkerboard(ptr, 16, 3));
    for (i =0;i< 16; ++i) {
        printf("%d ", ptr[i]);
    }
    wasm_free_u8(ptr, 16);

}
