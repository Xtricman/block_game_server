#include "oak_log.hpp"
namespace block {
    PyModuleDef module_def = {
        .m_base =PyModuleDef_HEAD_INIT,
        .m_name = "block",
        .m_doc = "block types",
        .m_size = -1,
    };

    PyMODINIT_FUNC PyInit__block_module(void) {
        PyObject * m;
        m = PyModule_Create(&module_def);
        if (m == NULL) return NULL;
        
        if (PyModule_Add(m, "oak_log", PyType_FromSpec(&oak_log::type_spec)) < 0) {
            Py_DECREF(m);
            return NULL;
        }

        return m;
    }
}