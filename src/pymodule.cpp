#include <Python.h>
#include "block/pymodule.hpp"
#include "block_component/pymodule.hpp"

static PyModuleDef global_module_def = {
    .m_base =PyModuleDef_HEAD_INIT,
    .m_name = "blockserver",
    .m_doc = "C++ written types",
    .m_size = -1,
};

PyMODINIT_FUNC
PyInit_blockserver(void)
{
    PyObject *m, *block_module, *block_component_module;

    m = PyModule_Create(&global_module_def);
    if (m == NULL)
        return NULL;

    block_module = block::PyInit__block_module();
    block_component_module = block_component::PyInit__block_component_module();
    if (block_module==NULL || block_component_module==NULL) return NULL;

    if (PyModule_Add(m, "block", block_module) < 0) {
        Py_DECREF(m);
        return NULL;
    }
    if (PyModule_Add(m, "block_component", block_component_module) < 0) {
        Py_DECREF(m);
        return NULL;
    }

    return m;
}