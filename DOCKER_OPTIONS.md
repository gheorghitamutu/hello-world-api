# Docker Runtime Options Summary

This repository provides multiple Dockerfile variants optimized for different use cases:

## 🚀 Available Dockerfiles

### 1. **Dockerfile** (Primary - Distroless)
- **Base**: `gcr.io/distroless/cc-debian12:latest`
- **Size**: 40.4MB
- **Security**: Maximum (no shell, minimal attack surface)
- **Debugging**: None (no shell access)
- **Use Case**: Production deployments requiring maximum security
- **Build**: `docker build -t hello-world-api .`

### 2. **Dockerfile.debian-slim** (Debugging)
- **Base**: `debian:12-slim`
- **Size**: 132MB
- **Security**: Good (minimal OS with security updates)
- **Debugging**: Full (procps, strace, vim, curl, debug script)
- **Use Case**: Development, debugging, production monitoring
- **Build**: `docker build -f Dockerfile.debian-slim -t hello-world-api:debug .`

### 3. **Dockerfile.alpine** (Experimental)
- **Base**: `alpine:latest`
- **Size**: 75MB
- **Security**: Good (minimal musl-based)
- **Debugging**: Limited (glibc compatibility issues)
- **Use Case**: Experimental (has compatibility issues)
- **Build**: `docker build -f Dockerfile.alpine -t hello-world-api:alpine .`

### 4. **Dockerfile.ubuntu** (Alternative)
- **Base**: `ubuntu:22.04`
- **Size**: ~110MB (estimated)
- **Security**: Good (LTS with security updates)
- **Debugging**: Full
- **Use Case**: Alternative to Debian
- **Build**: `docker build -f Dockerfile.ubuntu -t hello-world-api:ubuntu .`

### 5. **Dockerfile.distroless** (Reference)
- **Base**: `gcr.io/distroless/cc-debian12`
- **Size**: 40.4MB
- **Security**: Maximum
- **Debugging**: None
- **Use Case**: Reference implementation
- **Build**: `docker build -f Dockerfile.distroless -t hello-world-api:distroless .`

## 📊 Size Comparison

| Dockerfile | Base Image | Final Size | Security | Debugging | Recommended For |
|------------|------------|------------|----------|-----------|-----------------|
| **Dockerfile** | Distroless | **40.4MB** | ✅ Maximum | ❌ None | **Production** |
| **Dockerfile.debian-slim** | Debian Slim | **132MB** | ✅ Good | ✅ Full | **Development** |
| Dockerfile.alpine | Alpine | 75MB | ✅ Good | ⚠️ Limited | Experimental |
| Dockerfile.ubuntu | Ubuntu | ~110MB | ✅ Good | ✅ Full | Alternative |

## 🎯 Recommendations

### Production Deployment
```bash
# Maximum security, minimal attack surface
docker build -t hello-world-api:prod .
docker run -p 8080:8080 hello-world-api:prod
```

### Development & Debugging
```bash
# Full debugging capabilities
docker build -f Dockerfile.debian-slim -t hello-world-api:debug .
docker run -p 8080:8080 hello-world-api:debug

# Interactive debugging
docker exec -it container_name /app/debug.sh info
docker exec -it container_name /app/debug.sh test
docker exec -it container_name /app/debug.sh monitor
```

### OpenShift/Kubernetes
```bash
# Use debian-slim for better compatibility
oc new-build https://github.com/gheorghitamutu/hello-world-api \
  --docker-image=debian:12-slim \
  --name=hello-world-api
```

## 🔧 Features Comparison

### Distroless (Primary)
✅ Smallest size (40.4MB)
✅ Maximum security
✅ TRACE-level logging
❌ No shell access
❌ No debugging tools
❌ No package manager

### Debian Slim (Debug)
✅ Comprehensive debugging
✅ Debug script with monitoring
✅ TRACE-level logging
✅ Shell access
✅ Process monitoring
✅ API testing
⚠️ Larger size (132MB)

Choose based on your deployment requirements:
- **Production**: Use main Dockerfile (Distroless)
- **Development**: Use Dockerfile.debian-slim
- **Enterprise**: Consider Dockerfile.ubuntu
