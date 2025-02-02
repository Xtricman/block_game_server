#include <Python.h>
#include "orientation.hpp"

namespace block_component {
    PyModuleDef module_def = {
        .m_base =PyModuleDef_HEAD_INIT,
        .m_name = "block",
        .m_doc = "block_component types",
        .m_size = -1,
    };

    PyMODINIT_FUNC PyInit__block_component_module(void) {
        PyObject * m;
        m = PyModule_Create(&module_def);
        if (m == NULL) return NULL;
        
        if (PyModule_Add(m, "orientation", PyType_FromSpec(&orientation::type_spec)) < 0) {
            Py_DECREF(m);
            return NULL;
        }

        return m;
    }
}