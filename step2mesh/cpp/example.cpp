#include "step2mesh.h"
#include <iostream>
#include <fstream>
#include <vector>

using namespace step2mesh;

int main(int argv, char **argc) {
    if (argv != 3) {
        std::cerr << "usage: example <input> <output>" << std::endl;
        return 1;
    }

    const char *in_path = argc[1];
    const char *out_path = argc[2];

    std::cout << "in: " << in_path << std::endl;
    std::cout << "out: " << out_path << std::endl;

    std::ifstream in_file(in_path);
    if (!in_file.is_open()) {
        std::cerr << "failed to open input file" << std::endl;
        return 1;
    }

    in_file.seekg(0, std::ios::end);
    size_t in_size = in_file.tellg();
    in_file.seekg(0);

    char *step = new char[in_size + 1];
    in_file.read(step, in_size * sizeof(char));
    step[in_size] = 0;

    if (!in_file.good()) {
        std::cerr << "failed to read input file" << std::endl;
        return 1;
    }
    in_file.close();

    Mesh *mesh = mesh_from_step(step);
    if (mesh == nullptr) {
        std::cerr << "failed to parse step file" << std::endl;
        return 1;
    }
    delete[] step;

    MeshStats stats;
    mesh_get_stats(mesh, &stats);
    std::cout << stats.num_vertices << " vertices, " << stats.num_faces << " faces" << std::endl;

    std::vector<MeshPoint3d> verts(stats.num_vertices);
    std::vector<MeshIndexedTriangle> faces(stats.num_faces);
    mesh_export(mesh, verts.data(), faces.data());

    std::ofstream out_file(out_path);
    if (!out_file.is_open()) {
        std::cerr << "failed to open output file" << std::endl;
        return 1;
    }

    out_file << std::fixed;
    out_file.precision(9);
    out_file << "# generated by step2mesh" << '\n';

    for (auto &vert : verts) {
        out_file << "v " << vert.x << ' ' << vert.y << ' ' << vert.z << '\n';
    }
    for (auto &face: faces) {
        out_file << "f";
        for (int i = 0; i < 3; i++) out_file << ' ' << 1 + face.id[i];
        out_file << '\n';
    }

    out_file.close();

    return 0;
}