#pragma once

#include <cstdint>

#include "graphics.hpp"

const uint8_t* GetFont(const char c);

void WriteAscii(PixelWriter& writer, int x, int y, char c, const PixelColor& color);

void WriteString(PixelWriter& writer, const int x, const int y, const char* const s, const PixelColor& color);
