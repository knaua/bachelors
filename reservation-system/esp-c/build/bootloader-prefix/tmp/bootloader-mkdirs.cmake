# Distributed under the OSI-approved BSD 3-Clause License.  See accompanying
# file Copyright.txt or https://cmake.org/licensing for details.

cmake_minimum_required(VERSION 3.5)

file(MAKE_DIRECTORY
  "/home/alex/esp/esp-idf/components/bootloader/subproject"
  "/home/alex/Desktop/esp-http/build/bootloader"
  "/home/alex/Desktop/esp-http/build/bootloader-prefix"
  "/home/alex/Desktop/esp-http/build/bootloader-prefix/tmp"
  "/home/alex/Desktop/esp-http/build/bootloader-prefix/src/bootloader-stamp"
  "/home/alex/Desktop/esp-http/build/bootloader-prefix/src"
  "/home/alex/Desktop/esp-http/build/bootloader-prefix/src/bootloader-stamp"
)

set(configSubDirs )
foreach(subDir IN LISTS configSubDirs)
    file(MAKE_DIRECTORY "/home/alex/Desktop/esp-http/build/bootloader-prefix/src/bootloader-stamp/${subDir}")
endforeach()
if(cfgdir)
  file(MAKE_DIRECTORY "/home/alex/Desktop/esp-http/build/bootloader-prefix/src/bootloader-stamp${cfgdir}") # cfgdir has leading slash
endif()
