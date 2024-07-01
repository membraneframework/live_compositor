# Example: AWS EC2

This is an example configuration that shows how to deploy LiveCompositor to AWS EC2 instance with Terraform configuration.

All examples are located in [github.com/membraneframework-labs/live_compositor_deployment](https://github.com/membraneframework-labs/live_compositor_deployment) repository:
- `project` directory includes an example Membrane project that can consume multiple streams over RTMP and host the composed stream as a HLS playlist.
- `aws-ec2-terraform` directory includes an example Terraform+Packer configuration for building an AMI (Amazon Machine Image) and deploying it to EC2.

## Prerequisites

- Terraform
- Packer
- Elixir - required to build an example project
- FFmpeg - used to send/receive streams from/to the compositor

### Trade-off between CPU+GPU and CPU-only instances

- `GPU+CPU` - LiveCompositor is using `wgpu` (implementation of WebGPU standard written in Rust) for rendering. However, all decoding and encoding still happens on CPU. When running on GPU the rendering cost should be negligible when compared to the decoding/encoding.
- `CPU-only` - When running on CPU-only instance, all `WebGPU` code is emulated on CPU. Unless your encoder quality is set very high, rendering will use most of the CPU processing time.

Actual price-to-performance can vary, but in general CPU+GPU instances make more sense for fast encoder presets and complex rendering pipelines. However, CPU-only can be more optimal when using simple layouts and prioritizing quality over performance with slower preset.

### How to deploy

:::warning
The example configuration is using `us-east-1` region. If you want to use a different one make sure to change it both in Packer
and Terraform configuration. Specifically if you use EC2 instances with GPU, you might only have them available in some regions.
:::

Go to **aws-ec2-terraform/packer** directory and run `packer build membrane.pkr.hcl` to build AMI image with an example Membrane project.

The other `pkr.hcl` file in this directory (**standalone.pkr.hcl**) includes configuration for deploying just standalone LiveCompositor
instance, so you can also go that route, but the rest of this guide assumes you are using the provided Membrane project.

At the end of the process, the terminal will print AMI ID that will be needed later on (something like `ami-0e18e9d7b8c037ec2`).

Open **aws-ec2-terraform/main.tf**:
- Find `aws_instance.demo_instance` definition and update `ami` field to AMI ID from previous step.
- Change `instance_type` to e.g. `g4dn.xlarge` if you want to run on GPU (specifically NVIDIA T4 GPUs).

:::note
Instances with GPU like `g4dn.xlarge` are not available by default on AWS. You will need to request quota increase from AWS team to use them.
:::

### How to use

After everything is deployed you can open your AWS dashboard and find the public IP of the newly deployed instance.

To test the service, run in separate terminals:

- To receive the output stream
  ```
  ffplay http://YOUR_INSTANCE_IP:9001/index.m3u8
  ```
- To send example input stream
  ```
  ffmpeg -re -f lavfi -i testsrc
    -vf scale=1280:960 -vcodec libx264 \
    -profile:v baseline -preset fast -pix_fmt yuv420p \
    -f flv rtmp://YOUR_INSTANCE_IP:9000/app/stream_key
  ```
  - You can run this command multiple times with different path instead of `app/stream_key` to connect multiple streams
