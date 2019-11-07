#!/bin/bash -x

cd external/SDL2

wget https://www.libsdl.org/release/SDL2-2.0.8.zip
unzip -q SDL2-2.0.8.zip
mv SDL2-2.0.8 SDL2
rm SDL2-2.0.8.zip

wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.3.zip
unzip -q SDL2_image-2.0.3.zip
mv SDL2_image-2.0.3 SDL2_image
rm SDL2_image-2.0.3.zip
