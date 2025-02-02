#ifndef OAK_LOG
#define OAK_LOG
#include <Python.h>

namespace block::oak_log {
    typedef struct  {
    PyObject_HEAD
    } data_struct;

    PyType_Slot type_slot[] = {
        {0,  nullptr}
    };

    PyType_Spec type_spec = {
        .name = "oak_log",
        .basicsize = 0,
        .itemsize = 0,
        .slots = type_slot
    };

}
#endif