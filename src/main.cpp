#include <SDL.h>
#include <SDL_image.h>
#include <cstdio>
#include <android/log.h>

extern "C" {
void sdl2_main();
}

int main(int /*argc*/, char* /*argv*/[]) {
    sdl2_main();
    return 0;
}
