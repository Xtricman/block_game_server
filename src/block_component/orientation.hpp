#ifndef ORIENTATION
#define ORIENTATION
#include <Python.h>
namespace block_component::orientation {
    enum class data_struct {
        X, Y, Z
    };

    PyType_Slot type_slot[] = {
        {0, nullptr}
    };
    
    PyType_Spec type_spec = {
        .name = "orientation",
        .basicsize = 0,
        .itemsize = 0,
        .slots = type_slot
    };

}
#endif