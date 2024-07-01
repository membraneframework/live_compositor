# Example: AWS EC2

This is an example configuration that shows how to deploy LiveCompositor to AWS EC2 instance with Terraform configuration.

All examples are located in [github.com/membraneframework-labs/live_compositor_deployment](https://github.com/membraneframework-labs/live_compositor_deployment) repository:
- `project` directory includes example Membrane project that can consume multiple streams over RTMP and host the composed stream as a HLS playlist.
- `aws-ec2-terraform` directory includes an example Terraform configuration for building an AMI (Amazon Machine Image) and deploying it to EC2.

## Prerequisites

- Terraform
- Packer
- Elixir - required to build example project

### Trade-off between CPU+GPU and CPU-only instances

- `GPU+CPU` - LiveCompositor is using `wgpu` (implementation of WebGPU standard written in Rust) for all the rendering. However, all decoding and encoding still happens on CPU. When running on GPU the rendering cost should be negligible when compared to the decoding/encoding.
- `CPU-only` - When running on CPU only instance all `WebGPU` code is emulated on CPU. Unless your encoder quality is set very high rendering will use most of the CPU processing time.

Actual price-to-performance can vary, but in general CPU+GPU instances make more sense for fast encoder presets and complex rendering pipelines. However, CPU-only can be more optimal when using simple layouts and prioritizing quality over performance with slower preset.

### How to deploy

Go to **aws-ec2-terraform/packer** directory and run:

- `packer build membrane.pkr.hcl` - to build AMI image with example membrane project.
- `packer build standalone.pkr.hcl` - to build AMI image with just a LiveCompositor instance.

At the end of the process, the terminal will print AMI ID that will be needed later on (something like `ami-0e18e9d7b8c037ec2`).

Open **aws-ec2-terraform/main.tf**:
- Find `aws_instance.demo_instance` definition and update `ami` field to AMI ID from previous step.
- Change `instance_type` to e.g. `g4dn.xlarge` if you want to run on GPU (specifically NVIDIA T4 GPUs).

:::note

:::

