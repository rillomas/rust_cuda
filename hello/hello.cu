#include <stdio.h>
#include <cuda_runtime.h>
#include "hello.hpp"

__global__ void hello_kernel(int value) {
    printf("Hello from kernel! %d\n", value);
}

void execute_hello_kernel(int value) {
    printf("Hello from host! %d\n", value);
    hello_kernel<<<1,1>>>(value);
    // wait for the kernel to finish
    cudaDeviceSynchronize();
}
