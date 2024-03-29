cmake_minimum_required(VERSION 3.4.1)

string(TOLOWER ${CMAKE_BUILD_TYPE} ANDROID_BUILD_DIR)
set(DISTRIBUTION_DIR ${CMAKE_SOURCE_DIR}/android/distribution/android/SDL2/intermediates/ndkBuild)

set(SOURCE_FILES src/main.cpp)
set(SDL_LOCATION ${CMAKE_SOURCE_DIR}/external/SDL2)

add_library(SDL2 SHARED IMPORTED)
add_library(SDL2_image SHARED IMPORTED)

set_target_properties(SDL2 PROPERTIES IMPORTED_LOCATION
        ${DISTRIBUTION_DIR}/${ANDROID_BUILD_DIR}/obj/local/${ANDROID_ABI}/libSDL2.so)
set_target_properties(SDL2_image PROPERTIES IMPORTED_LOCATION
        ${DISTRIBUTION_DIR}/${ANDROID_BUILD_DIR}/obj/local/${ANDROID_ABI}/libSDL2_image.so)

include_directories(${SDL_LOCATION}/SDL2/include)
include_directories(${SDL_LOCATION}/SDL2_image)

add_library(main SHARED ${SDL_LOCATION}/SDL2/src/main/android/SDL_android_main.c ${SOURCE_FILES})

if (${ANDROID_ABI} STREQUAL "x86")
    set(RUST_TARGET "i686-linux-android")
else()
    set(RUST_TARGET "armv7-linux-androideabi")
endif()

# add imported target
add_library(librust STATIC IMPORTED)
# point the imported target at the real file
set_property(TARGET librust PROPERTY
        IMPORTED_LOCATION ${CMAKE_SOURCE_DIR}/rust-lib/target/${RUST_TARGET}/debug/librust.a)

target_link_libraries(main SDL2 SDL2_image librust log)
