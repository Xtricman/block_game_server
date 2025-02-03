#include <Python.h>
#include "block/block.hpp"

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
    if (m == nullptr)
        return nullptr;
    
    return m;
}