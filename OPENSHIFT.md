# OpenShift/OKD Deployment Guide

This guide explains how to deploy the Hello World API on OpenShift/OKD.

## Prerequisites

- OpenShift CLI (`oc`) installed and configured
- Access to an OpenShift/OKD cluster
- Docker or Podman for local builds (optional)

## Deployment Options

### Option 1: Using OpenShift BuildConfig (Recommended)

This approach uses OpenShift's Source-to-Image (S2I) capabilities:

```bash
# Create a new project
oc new-project hello-world-api

# Apply the BuildConfig and ImageStream
oc apply -f k8s/buildconfig.yaml

# Apply ConfigMap and Secrets
oc apply -f k8s/configmap.yaml

# Apply the Deployment and Service
oc apply -f k8s/deployment.yaml

# Check the build status
oc logs -f bc/hello-world-api-build

# Check deployment status
oc get pods -w
```

### Option 2: Using Pre-built Image

If you have a pre-built image in a registry:

```bash
# Create a new project
oc new-project hello-world-api

# Update the image reference in deployment.yaml
# Then apply the configurations
oc apply -f k8s/configmap.yaml
oc apply -f k8s/deployment.yaml
```

## Key OpenShift/OKD Optimizations

### 1. **Security Context**
- Uses non-root user (UID 1001)
- Drops all capabilities
- Runs with read-only root filesystem where possible

### 2. **Red Hat UBI Base Image**
- Uses `registry.access.redhat.com/ubi9/ubi-minimal` for better compatibility
- Officially supported by Red Hat
- Regular security updates

### 3. **Health Checks**
- **Liveness Probe**: Checks if the application is running
- **Readiness Probe**: Checks if the application is ready to serve traffic
- **Initial Delay**: 330 seconds to account for the 5-minute countdown

### 4. **Resource Management**
- Memory requests: 64Mi, limits: 128Mi
- CPU requests: 50m, limits: 100m
- Adjust based on your requirements

### 5. **Networking**
- Service exposes the application internally
- Route provides external access with TLS termination
- Automatic HTTPS redirect

## Configuration

### Environment Variables

The application supports these environment variables:

- `PORT`: Server port (default: 8080)
- `COUNTDOWN_SECONDS`: Startup countdown duration (default: 10, use 300 for development)
- `LOG_LEVEL`: Logging level
- `ENVIRONMENT`: Environment name

### Countdown Timer Configuration

The application includes a configurable countdown timer before starting the web server:

- **Production**: Set `COUNTDOWN_SECONDS=10` for fast startup
- **Development/Testing**: Set `COUNTDOWN_SECONDS=300` for extended observation
- **Disabled**: Set `COUNTDOWN_SECONDS=0` to skip countdown entirely

### Secrets Management

Store sensitive data in the Secret object:

```bash
# Create secrets
oc create secret generic hello-world-api-secrets \
  --from-literal=DATABASE_URL="your-database-url" \
  --from-literal=API_KEY="your-api-key"
```

## Monitoring and Debugging

### Check Application Logs
```bash
oc logs -f deployment/hello-world-api
```

### Check Events
```bash
oc get events --sort-by=.metadata.creationTimestamp
```

### Port Forward for Local Testing
```bash
oc port-forward service/hello-world-api-service 8080:8080
```

### Scale the Application
```bash
oc scale deployment hello-world-api --replicas=3
```

## Accessing the Application

After deployment, the application will be available at:

- **Internal**: `http://hello-world-api-service:8080`
- **External**: `https://hello-world-api-route-<project>.<cluster-domain>`

### API Endpoints

- `GET /` - Main page
- `GET /hello` - Hello World message
- `GET /health` - Health check endpoint

## Troubleshooting

### CrashLoopBackOff Issues

If you're experiencing CrashLoopBackOff, check these common causes:

#### 1. **Countdown Timer Too Long**
```bash
# Check current countdown setting
oc get deployment hello-world-api -o yaml | grep -A5 COUNTDOWN_SECONDS

# Fix: Set shorter countdown for production
oc set env deployment/hello-world-api COUNTDOWN_SECONDS=10

# Or use the production deployment which has optimized settings
oc apply -f k8s/deployment.yaml
```

#### 2. **Health Check Timing**
```bash
# Check probe configuration
oc describe deployment hello-world-api

# The startup probe should allow enough time for countdown + server start
# Production config: 10s countdown + 20s buffer = 30s max startup time
# Development config: 300s countdown + 50s buffer = 350s max startup time
```

#### 3. **Resource Limits**
```bash
# Check if pod is being killed due to resource limits
oc describe pod <pod-name>

# Look for events like "OOMKilled" or "Evicted"
# Increase memory limits if needed:
oc patch deployment hello-world-api -p '{"spec":{"template":{"spec":{"containers":[{"name":"hello-world-api","resources":{"limits":{"memory":"256Mi"}}}]}}}}'
```

### Deployment Options

#### Quick Start (Production - Fast Startup)
```bash
# Uses 10-second countdown, optimized health checks
oc apply -f k8s/deployment.yaml
```

#### Development (Extended Countdown)
```bash
# Uses 300-second countdown for debugging
oc apply -f k8s/deployment-dev.yaml
```

### Common Issues

1. **Long Startup Time**: The application has a configurable countdown before serving requests
2. **Health Check Failures**: Ensure probes have sufficient initial delay based on countdown setting
3. **Permission Issues**: Verify the application runs as non-root user (UID 1001)
4. **Build Failures**: Check BuildConfig logs for compilation errors
5. **Port Conflicts**: Application will try alternative port 3000 if 8080 fails

### Useful Commands

```bash
# Check pod status
oc get pods -l app=hello-world-api

# Describe pod for detailed information
oc describe pod <pod-name>

# Get route URL
oc get route hello-world-api-route -o jsonpath='{.spec.host}'

# View build logs
oc logs -f bc/hello-world-api-build
```

## Security Considerations

- Application runs as non-root user (UID 1001)
- Uses Security Context Constraints (SCC)
- Network policies can be applied for additional isolation
- Secrets are mounted as environment variables or files
- TLS termination at the route level

## Production Recommendations

1. **Resource Limits**: Adjust CPU/memory based on load testing
2. **Horizontal Pod Autoscaler**: Add HPA for automatic scaling
3. **Persistent Storage**: Add if your application needs persistent data
4. **Monitoring**: Integrate with Prometheus/Grafana
5. **Logging**: Configure centralized logging with ELK stack
6. **Backup**: Implement backup strategies for configuration and data

## Debugging in OpenShift

### Debug Mode Deployment

For debugging purposes, use the special debug deployment that keeps the container alive:

```bash
# Deploy in debug mode
oc apply -f k8s/deployment-debug.yaml

# Get pod name
POD_NAME=$(oc get pods -l app=hello-world-api-debug -o jsonpath='{.items[0].metadata.name}')

# Exec into the pod for debugging
oc exec -it $POD_NAME -- /bin/sh
```

### Available Debug Tools

The container includes these debugging tools:
- `ps`, `top`, `pgrep` - Process monitoring
- `curl` - API testing
- `strace` - System call tracing
- `vim`, `less` - Text viewing/editing
- `netstat`, `ss` - Network monitoring (where available)

### Debug Script Usage

Inside the pod, use the built-in debug script:

```bash
# Show all options
./debug.sh

# Run app with exit code monitoring
./debug.sh run

# Test API endpoints
./debug.sh test

# Show system information
./debug.sh info

# Continuous monitoring
./debug.sh monitor
```

### Common Debug Scenarios

#### CrashLoopBackOff Investigation
```bash
# Check pod logs
oc logs $POD_NAME --previous

# Exec into debug pod
oc exec -it $POD_NAME -- ./debug.sh run

# Monitor process exit codes
oc exec -it $POD_NAME -- /bin/sh -c 'echo $$; exec /app/hello-world-api'
```

#### Network Connectivity Issues
```bash
# Test from inside pod
oc exec -it $POD_NAME -- curl http://localhost:8080/health

# Check network interfaces
oc exec -it $POD_NAME -- ip addr show

# Check listening ports
oc exec -it $POD_NAME -- netstat -tlnp
```

#### Performance Issues
```bash
# Monitor resource usage
oc exec -it $POD_NAME -- htop

# Check memory
oc exec -it $POD_NAME -- free -h

# Monitor process
oc exec -it $POD_NAME -- top
```

For detailed debugging instructions, see [DEBUG.md](DEBUG.md).
