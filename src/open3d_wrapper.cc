#include "open3d_wrapper.h"

#include "open3d/Open3D.h"

void test_fn() {
    open3d::PrintOpen3DVersion();
    auto sphere = open3d::geometry::TriangleMesh::CreateSphere(1.0);
    sphere->ComputeVertexNormals();
    sphere->PaintUniformColor({0.0, 1.0, 0.0});
    open3d::visualization::DrawGeometries({sphere});
}

