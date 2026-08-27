# 只允许操作本地 kind 集群，避免误触其它环境
allow_k8s_contexts('kind-asterpi')

# ---------- 镜像构建 ----------
# 注意：第二个位置参数是「构建上下文」，Dockerfile 必须用 dockerfile= 显式指定
docker_build(
    'asterpi',                                  # 镜像 ref，需与 k8s 清单里的 image 一致
    context='.',
    dockerfile='./registry/Dockerfile.AsterPI',
)

docker_build(
    'docs',
    context='.',
    dockerfile='./registry/Dockerfile.docs',
)

# ---------- 加载 K8s 清单 ----------
k8s_yaml([
    'k8s/asterpi.yaml',
    'k8s/docs.yaml',
])

# ---------- 资源与端口转发 ----------
# 字符串格式为 "本地端口:容器端口"
k8s_resource('asterpi', port_forwards=['3000:3000'])
k8s_resource('docs',    port_forwards=['8080:80'])