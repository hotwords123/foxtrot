
namespace step2mesh {

struct Mesh {
    Mesh() = delete;
    Mesh(const Mesh &) = delete;
};

struct MeshStats {
    int num_vertices;
    int num_faces;
};

struct MeshPoint3d {
    double x;
    double y;
    double z;
};

struct MeshIndexedTriangle {
    int id[3];
};


extern "C" {

Mesh *mesh_from_step(const char *str);

void mesh_free(Mesh *ptr_mesh);

void mesh_get_stats(const Mesh *ptr_mesh, MeshStats *ptr_stat);

void mesh_export(const Mesh *ptr_mesh, MeshPoint3d *ptr_verts, MeshIndexedTriangle *ptr_faces);

} // extern "C"

} // namespace step2mesh
