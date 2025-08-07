#!/bin/sh

# Debug script for hello-world-api
# This script helps you debug the Rust application in the container

echo "=== Hello World API Debug Script ==="
echo

# Function to run the app and capture its exit code
run_app_with_debug() {
    echo "Starting hello-world-api with debugging..."
    echo "Process will be monitored for output and exit codes"
    echo
    
    # Start the application in background and capture PID
    /app/hello-world-api &
    APP_PID=$!
    
    echo "Application started with PID: $APP_PID"
    echo
    
    # Monitor the process
    while kill -0 $APP_PID 2>/dev/null; do
        echo "App is running (PID: $APP_PID)..."
        sleep 2
    done
    
    # Wait for the process to finish and get exit code
    wait $APP_PID
    EXIT_CODE=$?
    
    echo
    echo "Application finished with exit code: $EXIT_CODE"
    return $EXIT_CODE
}

# Function to show process information
show_process_info() {
    echo "=== Process Information ==="
    echo "Running processes:"
    ps aux
    echo
    echo "Memory usage:"
    free -h 2>/dev/null || cat /proc/meminfo | head -10
    echo
    echo "Disk usage:"
    df -h
    echo
    echo "Load average:"
    cat /proc/loadavg
    echo
}

# Function to test the API endpoints
test_api() {
    echo "=== API Testing ==="
    echo "Testing /health endpoint:"
    curl -s -w "\nHTTP Status: %{http_code}\n" http://localhost:8080/health 2>/dev/null || echo "Failed to connect (curl may be curl-minimal with limited options)"
    echo
    echo "Testing /hello endpoint:"
    curl -s -w "\nHTTP Status: %{http_code}\n" http://localhost:8080/hello 2>/dev/null || echo "Failed to connect"
    echo
    echo "Testing / endpoint:"
    curl -s -w "\nHTTP Status: %{http_code}\n" http://localhost:8080/ 2>/dev/null || echo "Failed to connect"
    echo
}

# Function to show environment variables
show_env() {
    echo "=== Environment Variables ==="
    echo "COUNTDOWN_SECONDS=${COUNTDOWN_SECONDS:-not set}"
    echo "RUST_LOG=${RUST_LOG:-not set}"
    echo "RUST_BACKTRACE=${RUST_BACKTRACE:-not set}"
    echo
    echo "All environment variables:"
    env | sort
    echo
}

# Function to show network information
show_network() {
    echo "=== Network Information ==="
    echo "Listening ports:"
    netstat -tlnp 2>/dev/null || ss -tlnp 2>/dev/null || echo "netstat/ss not available"
    echo
    echo "Network interfaces:"
    ip addr show 2>/dev/null || cat /proc/net/dev
    echo
    echo "Open files (if available):"
    if command -v lsof >/dev/null 2>&1; then
        lsof | head -20
    else
        echo "lsof not available - showing /proc/*/fd for main process:"
        ls -la /proc/self/fd/ 2>/dev/null || echo "Cannot access file descriptors"
    fi
    echo
}

# Function to trace system calls (if strace is available)
trace_app() {
    echo "=== System Call Tracing ==="
    echo "Running application with strace (if available)..."
    if command -v strace >/dev/null 2>&1; then
        strace -f -e trace=file,network,signal /app/hello-world-api
    else
        echo "strace not available, running normally..."
        /app/hello-world-api
    fi
}

# Main menu
case "${1:-menu}" in
    "run")
        run_app_with_debug
        ;;
    "info")
        show_process_info
        ;;
    "test")
        test_api
        ;;
    "env")
        show_env
        ;;
    "network")
        show_network
        ;;
    "trace")
        trace_app
        ;;
    "monitor")
        echo "=== Continuous Monitoring ==="
        echo "Starting app and monitoring..."
        run_app_with_debug &
        MONITOR_PID=$!
        
        while kill -0 $MONITOR_PID 2>/dev/null; do
            clear
            show_process_info
            test_api
            sleep 5
        done
        ;;
    *)
        echo "Usage: $0 {run|info|test|env|network|trace|monitor}"
        echo
        echo "Commands:"
        echo "  run      - Run the app with debug monitoring"
        echo "  info     - Show process and system information"
        echo "  test     - Test API endpoints"
        echo "  env      - Show environment variables"
        echo "  network  - Show network information"
        echo "  trace    - Run app with system call tracing"
        echo "  monitor  - Continuous monitoring (run + info + test)"
        echo
        echo "Examples:"
        echo "  ./debug.sh run       # Run app and monitor exit code"
        echo "  ./debug.sh info      # Show system info"
        echo "  ./debug.sh test      # Test API endpoints"
        ;;
esac
