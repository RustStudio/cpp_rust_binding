#include "lib.rs.h"
#include <iostream>
#include <vector>

int main() {
    auto object = org::blobstore::new_object();

    auto object_name = object->get_name();

    std::cout << "Object name: " << static_cast<std::string>(object_name) << std::endl;

    return 0;
}
