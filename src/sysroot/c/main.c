#include "rt/intrinsics/io.h"

struct Person;

struct Person {
    const char *name;
};

void person_say_name(struct Person *self);

int main(void) {
    struct Person p = {.name = "Elodie"};
    person_say_name(&p);
}

void person_say_name(struct Person *self) {
//    printf("Hello, %s!", self->name);
    rt_intrinsics_io_print("Hello hello");
}
