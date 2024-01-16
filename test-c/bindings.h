#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum PMouseButton {
  LeftButton,
  RightButton,
  MiddleButton,
  NoButton,
} PMouseButton;

void createWindow(float width, float height);

float width(void);

float height(void);

float mouseX(void);

float mouseY(void);

enum PMouseButton mouseButton(void);
