#include "step2mesh.h"
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

using namespace step2mesh;

bool ends_with(const std::string &str, const std::string &suffix) {
    return str.length() >= suffix.length() &&
        0 == str.compare(str.length() - suffix.length(), suffix.length(), suffix);
}

int main(int argv, char **argc) {
    if (argv != 3) {
        std::cerr << "usage: example2 <input> <output>" << std::endl;
        return 1;
    }

    std::string in_path = argc[1];
    std::string out_path = argc[2];

    std::cout << "in: " << in_path << std::endl;
    std::cout << "out: " << out_path << std::endl;

    Mesh *mesh = mesh_read_step(in_path.c_str());
    if (mesh == nullptr) {
        std::cerr << "failed to parse step file" << std::endl;
        return 1;
    }

    MeshStats stats;
    mesh_get_stats(mesh, &stats);
    std::cout << stats.num_vertices << " vertices, " << stats.num_faces << " faces" << std::endl;

    if (ends_with(out_path, ".obj")) {
        mesh_save_obj(mesh, out_path.c_str());
    } else if (ends_with(out_path, ".stl")) {
        mesh_save_stl(mesh, out_path.c_str());
    } else {
        std::cerr << "Unrecognized extension" << std::endl;
    }

    mesh_free(mesh);

    return 0;
}