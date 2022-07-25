# Awesome Rust Cloud Native

[![Awesome](https://cdn.rawgit.com/sindresorhus/awesome/d7305f38d29fed78fa85652e3a63e154dd8e8829/media/badge.svg)](https://github.com/sindresorhus/awesome)
![License](https://img.shields.io/github/license/awesome-rust-cloud-native/awesome-rust-cloud-native)

<img src="./logo/img/awesome-rust-cloud-native-logo.png" alt="Awesome Rust Cloud Native logo, which features the Ferris crab mascot on a white cloud with a light blue background." width="200">

## Announcement

"Rust Cloud Native" has been converted to "Awesome Rust Cloud Native".
For more details, please read the [announcement](./ANNOUNCEMENT.md).

## Contributing

Please refer to [CONTRIBUTING.md](./CONTRIBUTING.md) for more details!

This project exists to showcase cloud native Rust projects and provide a collection of cloud native Rust resources.
We feature projects that exist in our space for the purpose of connecting our greater community, but _we are not affiliated with these projects unless specified_.
Explicit consent from maintainers and owners must be given for affiliation.
Thus, you see a package or project here that is no longer maintained or is not a good fit, please submit a pull request to improve this file.
Thank you!

## Contents

- [Applications and Services](#applications-and-services)
- [Libraries](#libraries)

### Applications and Services

- **[apache/incubator-teaclave](https://github.com/apache/incubator-teaclave)**: open source universal secure computing platform, making computation on privacy-sensitive data safe and simple
- **[bottlerocket-os/bottlerocket](https://github.com/bottlerocket-os/bottlerocket)**: an operating system designed for hosting containers
- **[containers/krunvm](https://github.com/containers/krunvm)**: manage lightweight VMs created from OCI images
- **[containers/youki](https://github.com/containers/youki)**: a container runtime written in Rust
- **[datafuselabs/datafuse](https://github.com/datafuselabs/datafuse)**: A Modern Real-Time Data Processing & Analytics DBMS with Cloud-Native Architecture, built to make the Data Cloud easy
- **[dragonflyoss/image-service](https://github.com/dragonflyoss/image-service)**: container image service focused on speed, space, network efficiency and data integrity, replacement of OCI
- **[firecracker-microvm/firecracker](https://github.com/firecracker-microvm/firecracker)**: secure and fast microVMs for serverless computing
- **[infinyon/fluvio](https://github.com/infinyon/fluvio)**: A cloud-native real-time data streaming platform with in-line computation capabilities
- **[kata-containers/kata-containers](https://github.com/kata-containers/kata-containers)**: VM-based container runtime with the security of virtual machine and speed of container
- **[krustlet/krustlet](https://github.com/krustlet/krustlet)**: Kubernetes Rust Kubelet
- **[kube-rs/controller-rs](https://github.com/kube-rs/controller-rs)**: a Kubernetes example controller
- **[kube-rs/version-rs](https://github.com/kube-rs/version-rs)**: example Kubernetes reflector and web server
- **[kubewarden/policy-server](https://github.com/kubewarden/policy-server)**: webhook server that evaluates WebAssembly policies to validate Kubernetes requests
- **[linkerd/linkerd2-proxy](https://github.com/linkerd/linkerd2-proxy)**: a purpose-built proxy for the Linkerd service mesh
- **[openebs/mayastor](https://github.com/openebs/mayastor)**: A cloud native declarative data plane in containers for containers
- **[rancher-sandbox/lockc](https://github.com/rancher-sandbox/lockc)**: eBPF-based MAC security audit for container workloads
- **[tikv/tikv](https://github.com/tikv/tikv)**: distributed transactional key-value database
- **[tremor-rs/tremor-runtime](https://github.com/tremor-rs/tremor-runtime)**: an event processing system that supporting complex workflows such as aggregation, rollups, an ETL language, and a query language
- **[valeriansaliou/sonic](https://github.com/valeriansaliou/sonic)**: fast, lightweight & schema-less search backend
- **[WasmEdge/WasmEdge](https://wasmedge.org)**: WasmEdge is a high-performance WebAssembly (Wasm) Virtual Machine (VM) runtime, which enables serverless functions to be embedded into any software platform; from cloud's edge to SaaS to automobiles

### Libraries

- **[containers/libkrun](https://github.com/containers/libkrun)**: a dynamic library providing Virtualization-based process isolation capabilities
- **[datafuselabs/opendal](https://github.com/datafuselabs/opendal)**: access different storage services painlessly and efficiently with no vendor lock-in
- **[deislabs/runwasi](https://github.com/deislabs/runwasi)**: a project to facilitate running wasm workloads managed by containerd either directly (ie. through ctr) or as directed by Kubelet via the CRI plugin
- **[kube-rs/kube-rs](https://github.com/kube-rs/kube-rs)**: Kubernetes Rust client and async controller runtime
- **[olix0r/kubert](https://github.com/olix0r/kubert)**: Kubernetes runtime helpers based on kube-rs
- **[open-telemetry/opentelemetry-rust](https://github.com/open-telemetry/opentelemetry-rust)**: OpenTelemetry is a set of APIs, SDKs, tooling and integrations that are designed for the creation and management of telemetry data such as traces, metrics, and logs.
- **[passcod/cni-plugins](https://github.com/passcod/cni-plugins)**: crate/framework to write CNI (container networking) plugins in Rust (includes a few custom plugins as well)
- **[qovery/engine](https://github.com/Qovery/engine)**: An open-source abstraction layer library that turns easy apps deployment on AWS, GCP, Azure, and other Cloud providers

## Licenses

All contents of this repository are licensed under the [MIT](https://opensource.org/licenses/MIT) license _except_ for the [logo](./logo) directory.
The logo directory and its contents (including the logo itself) are licensed under the [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) license.
