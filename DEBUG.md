# Debugging the Hello World API in Kubernetes/OpenShift

## Overview

The enhanced Dockerfile now includes debugging tools and utilities to help you monitor and debug your Rust application in a pod environment.

## Added Debugging Tools

The container now includes these packages:
- **procps-ng**: Provides `ps`, `top`, `pgrep`, `pkill`
- **util-linux**: Provides `kill`, and other utilities
- **findutils**: Provides `find`, `xargs`
- **which**: Locate commands
- **curl**: Test HTTP endpoints
- **vim-minimal**: Basic text editor
- **less**: Pager for viewing files
- **strace**: System call tracer

Note: Some tools like `htop` and `lsof` are not available in UBI minimal, but alternatives are provided in the debug script.

## Debugging Approaches

### 1. Debug Mode Deployment

Use the debug deployment that sleeps instead of running the app:

```bash
# Build and deploy debug version
oc apply -f k8s/deployment-debug.yaml

# Get pod name
POD_NAME=$(oc get pods -l app=hello-world-api-debug -o jsonpath='{.items[0].metadata.name}')

# Exec into the pod
oc exec -it $POD_NAME -- /bin/sh
```

### 2. Using the Debug Script

Inside the pod, use the included debug script:

```bash
# Show all options
./debug.sh

# Run app with monitoring and exit code capture
./debug.sh run

# Show system information
./debug.sh info

# Test API endpoints
./debug.sh test

# Show environment variables
./debug.sh env

# Show network information
./debug.sh network

# Run with system call tracing
./debug.sh trace

# Continuous monitoring
./debug.sh monitor
```

### 3. Manual Process Monitoring

```bash
# Run the app in background
/app/hello-world-api &
APP_PID=$!

# Monitor the process
ps aux | grep hello-world-api

# Check if process is still running
kill -0 $APP_PID && echo "Running" || echo "Stopped"

# Get exit code when it finishes
wait $APP_PID
echo "Exit code: $?"

# Check resource usage
top -p $APP_PID

# Check open files and network connections
lsof -p $APP_PID

# Check system calls (if needed)
strace -p $APP_PID
```

### 4. API Testing

```bash
# Test endpoints from within the pod
curl http://localhost:8080/health
curl http://localhost:8080/hello
curl http://localhost:8080/

# Check HTTP response codes
curl -w "HTTP Status: %{http_code}\n" http://localhost:8080/health
```

### 5. Log Analysis

```bash
# View pod logs from outside
oc logs -f $POD_NAME

# View logs with timestamps
oc logs $POD_NAME --timestamps

# View previous container logs (if crashed)
oc logs $POD_NAME --previous
```

### 6. Environment Debugging

```bash
# Check environment variables
env | grep -E "(COUNTDOWN|RUST|PORT)"

# Test with different countdown values
COUNTDOWN_SECONDS=5 /app/hello-world-api

# Enable debug logging
RUST_LOG=debug /app/hello-world-api

# Enable backtraces
RUST_BACKTRACE=1 /app/hello-world-api
```

## Common Debugging Scenarios

### App Won't Start
```bash
# Check if binary is executable
ls -la /app/hello-world-api

# Try running directly and check output
/app/hello-world-api

# Check for missing dependencies
ldd /app/hello-world-api
```

### App Crashes
```bash
# Run with backtrace
RUST_BACKTRACE=full /app/hello-world-api

# Use strace to see system calls
strace /app/hello-world-api

# Check core dumps (if enabled)
ls -la /tmp/core*
```

### Network Issues
```bash
# Check if port is bound
netstat -tlnp | grep 8080
ss -tlnp | grep 8080

# Test local connectivity
curl -v http://localhost:8080/health

# Check network interfaces
ip addr show
```

### Performance Issues
```bash
# Monitor resource usage
htop

# Check memory usage
free -h

# Monitor in real-time
watch -n 1 'ps aux | grep hello-world-api'
```

## Building and Testing

```bash
# Build with debug tools
docker build -t hello-world-api:debug .

# Test locally with debug tools
docker run -it --rm hello-world-api:debug /bin/sh

# Run the debug script
docker run -it --rm hello-world-api:debug ./debug.sh run
```

## Tips

1. **Use debug deployment for investigation**: The debug deployment keeps the container alive
2. **Monitor exit codes**: The debug script captures and reports exit codes
3. **Check environment variables**: Many issues are environment-related
4. **Use strace for deep debugging**: System call tracing can reveal low-level issues
5. **Test endpoints locally**: Use curl within the container to test connectivity
6. **Monitor resources**: Use htop and ps to check resource usage
7. **Check logs continuously**: Use `oc logs -f` to monitor real-time output
