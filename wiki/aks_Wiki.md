# SigmaOS-aks Component Wiki

## Overview
SigmaOS-aks provides Kubernetes Service and container orchestration support for the SigmaOS operating system. It demonstrates the container management subsystem and Kubernetes integration within the SigmaOS ecosystem, following the principles of clear abstraction and deterministic container initialization.

## Principles
- Single Responsibility
- Container Abstraction
- Fail-Fast on Container Initialization Errors
- Deterministic Container Setup
- Clear Abstraction Over Container APIs
- Composition over Inheritance for container extensions

## Architecture
SigmaOS-aks follows a container pattern:
- **Container Core**: Provides container management, orchestration, and Kubernetes API integration
- **Kubernetes Integration**: Supports Kubernetes API, pod management, and cluster operations
- **Container Lifecycle**: Supports container creation, scaling, and lifecycle management
- **Error Handling**: Fail-fast on invalid container configurations or unsupported container types

## API
The component provides the following public APIs:
- `container::k8s::deploy(deployment)` - Deploy a containerized application to Kubernetes
- `container::k8s::scale(deployment, replicas)` - Scale a deployment to given replica count
- `container::k8s::status::get()` - Get container status and Kubernetes cluster information
- `container::k8s::pod::create(pod_spec)` - Create a Kubernetes pod with given spec
- Error types for container failure scenarios or unsupported container types

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific container configuration
2. **Act**: Execute the code under test (deployment, scaling)
3. **Assert**: Verify the expected container results

Example:
```rust
#[test]
fn test_container_deploy_aaa() {
    // Arrange
    let deployment = DeploymentSpec::new("my-app").with_replicas(3);
    
    // Act
    let result = container::k8s::deploy(deployment);
    
    // Assert
    assert!(result.is_ok());
    let deployed = result.unwrap();
    assert!(deployed.is_running());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- Container initialization is best-effort; gracefully degrades on unsupported configurations
- Comprehensive error types for all container failure scenarios
- Unit tests provided for container management and Kubernetes integration
- Integration tests for full container system with actual Kubernetes orchestration

## TODO
- Add support for additional Kubernetes resources (Services, Ingress, ConfigMaps)
- Expand orchestration capabilities (rolling updates, self-healing, resource quotas)
- Add cluster management features (cluster creation, node management, cluster scaling)
- Create integration tests with actual Kubernetes clusters and container orchestration
- Implement container persistence and orchestration state management