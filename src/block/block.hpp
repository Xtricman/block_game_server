#ifndef BLOCK_HPP
#define BLOCK_HPP


#include <Python.h>
#include "oak_log.hpp"

namespace block {
    struct block {
        char *block_id;
        union {
            oak_log a;
        } data;
    };
}


#endif