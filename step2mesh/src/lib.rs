pub mod step2mesh {
    use std::os::raw::{c_char, c_int, c_double};
    use std::ffi::CStr;
    use std::panic::catch_unwind;

    use triangulate::mesh::Mesh;
    use triangulate::triangulate::triangulate;
    use step::step_file::StepFile;

    #[repr(C)]
    pub struct MeshStats {
        pub num_vertices: c_int,
        pub num_faces: c_int,
    }

    #[repr(C)]
    pub struct MeshPoint3d {
        pub x: c_double,
        pub y: c_double,
        pub z: c_double,
    }

    #[repr(C)]
    pub struct MeshIndexedTriangle {
        pub id: [c_int; 3]
    }

    #[no_mangle]
    pub extern "C" fn mesh_from_step(str: *const c_char) -> *mut Mesh {
        catch_unwind(|| {
            let data = unsafe {
                CStr::from_ptr(str).to_bytes()
            };
            let flattened = StepFile::strip_flatten(data);
            let entities = StepFile::parse(&flattened);
            let (mesh, _) = triangulate(&entities);
            Box::into_raw(Box::new(mesh))
        }).unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub extern "C" fn mesh_free(mesh: *mut Mesh) {
        unsafe { Box::from_raw(mesh); }
    }

    #[no_mangle]
    pub extern "C" fn mesh_get_stats(ptr_mesh: *const Mesh, ptr_stat: *mut MeshStats) {
        let mesh = unsafe { &*ptr_mesh };
        let stat = unsafe { &mut *ptr_stat };
        stat.num_vertices = mesh.verts.len() as c_int;
        stat.num_faces = mesh.triangles.len() as c_int;
    }

    #[no_mangle]
    pub extern "C" fn mesh_export(ptr_mesh: *const Mesh, ptr_verts: *mut MeshPoint3d, ptr_faces: *mut MeshIndexedTriangle) {
        let mesh = unsafe { &*ptr_mesh };
        let verts = unsafe {
            std::slice::from_raw_parts_mut(ptr_verts, mesh.verts.len())
        };
        let faces = unsafe {
            std::slice::from_raw_parts_mut(ptr_faces, mesh.triangles.len())
        };
        for (i, tri) in mesh.triangles.iter().enumerate() {
            for j in 0..3 {
                faces[i].id[j] = tri.verts[j] as c_int;
            }
        }
        for (i, vert) in mesh.verts.iter().enumerate() {
            verts[i].x = vert.pos.x;
            verts[i].y = vert.pos.y;
            verts[i].z = vert.pos.z;
        }
    }
}