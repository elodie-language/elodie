#include "core/type/type.h"

typedef struct type tok;

const tok type_any = (tok) {.id = 0};
const tok type_nil = (tok) {.id = 1};
const tok type_object = (tok) {.id = 2};
const tok type_number = (tok) {.id = 3};
const tok type_string = (tok) {.id = 4};
const tok type_unit = (tok) {.id = 5};

bool
type_equal(struct type lhs, struct type rhs) {
    return lhs.id == rhs.id;
}
