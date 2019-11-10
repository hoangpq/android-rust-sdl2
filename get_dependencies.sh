#!/bin/bash -x

cd external/SDL2

wget https://www.libsdl.org/release/SDL2-2.0.9.zip
unzip -q SDL2-2.0.9.zip
mv SDL2-2.0.9 SDL2
rm SDL2-2.0.9.zip

wget https://www.libsdl.org/projects/SDL_image/release/SDL2_image-2.0.5.zip
unzip -q SDL2_image-2.0.5.zip
mv SDL2_image-2.0.5 SDL2_image
rm SDL2_image-2.0.5.zip
