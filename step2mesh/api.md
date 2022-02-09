# step2mesh API

## 简介

解析 STEP 格式文件并将其中的实体网格化。

## 编译

1. [安装 Rust 环境](https://doc.rust-lang.org/cargo/getting-started/installation.html)。
2. `cargo build --package step2mesh`，输出目录为 `target/debug`。

## 接口

头文件 `cpp/step2mesh.h` 中包含对下列数据结构与导出函数的声明，封装在命名空间 `step2mesh` 中。

### 数据结构

#### `Mesh`
```cpp
struct Mesh {
  // opaque data
};
```
在 Rust 代码中存储网格模型数据，内部结构对 C++ 端不透明。只能使用 `Mesh*` 通过导出接口进行间接访问和操作。

#### `MeshStats`
```cpp
struct MeshStats {
    int num_vertices; // 顶点数量
    int num_faces;    // 三角形数量
};
```
包含网格模型的统计信息。

#### `MeshPoint3d`
```cpp
struct MeshPoint3d {
    double x;
    double y;
    double z;
};
```
存储一个三维空间中的点。

#### `MeshIndexedTriangle`
```cpp
struct MeshIndexedTriangle {
    int id[3]; // 顶点对应的编号
};
```
存储一个三角形，用顶点对应的编号表示（编号从 0 开始）。

### 导出接口

#### `mesh_from_step`
```cpp
Mesh *mesh_from_step(const char *str);
```
解析 STEP 格式并对其中实体进行网格化。

##### 参数
- `str`：包含 STEP 格式内容的空终止字符串。

##### 返回值
指向内部 `Mesh` 对象的指针。如果解析失败，则返回空指针。

#### `mesh_free`
```cpp
void mesh_free(Mesh *ptr_mesh);
```
释放 `Mesh` 对象的资源。

##### 参数
- `ptr_mesh`：指向 `Mesh` 对象的指针。调用后，不能再将 `ptr_mesh` 作为调用参数。

#### `mesh_get_stats`
```cpp
void mesh_get_stats(const Mesh *ptr_mesh, MeshStats *ptr_stat);
```
获取 `Mesh` 对象的统计信息，以便分配导出数据所需的内存资源。

##### 参数
- `ptr_mesh`：指向 `Mesh` 对象的指针。
- `ptr_stat`：指向要写入的 `MeshStats` 对象的指针。

#### `mesh_export`
```cpp
void mesh_export(const Mesh *ptr_mesh, MeshPoint3d *ptr_verts, MeshIndexedTriangle *ptr_faces);
```
导出 `Mesh` 对象中存储的所有顶点和面的信息。

顶点的顺序与 `MeshIndexedTriangle` 结构中顶点的编号顺序相同。

##### 参数
- `ptr_mesh`：指向 `Mesh` 对象的指针。
- `ptr_verts`：指向要写入所有顶点信息的 `MeshPoint3d` 数组的指针。
- `ptr_faces`：指向要写入所有面信息的 `MeshIndexedTriangle` 数组的指针。

## 样例代码

样例 C++ 程序位于 `cpp/example.cpp`，它可以将 STEP 格式转换为 OBJ 格式。

GCC 编译命令：`g++ example.cpp -o example -lstep2mesh -L../../target/debug`。暂未在 MSVC 下测试。

运行前需要手动把生成的动态链接库复制到同一目录下。

命令格式：`example <input> <output>`，其中 `input` 是输入的 STEP 文件路径，`output` 是输出的 OBJ 文件路径。
