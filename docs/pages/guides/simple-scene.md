import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';
import SimpleSceneEmpty from "./simple_scene_1.webp"
import SimpleSceneOverflow from "./simple_scene_2.webp"
import SimpleSceneFitted from "./simple_scene_3.webp"
import SimpleSceneBothInputs from "./simple_scene_4.webp"
import SimpleSceneAbsolutePosition from "./simple_scene_5.webp"

# Simple scene

This guide will explain how to create simple scene that is combining input streams in a simple layout into single output stream.

## Configure inputs and output

### Start the compositor

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    [Start the compositor server](../deployment/overview.md). Check out [configuration page](../deployment/configuration.md) for available configuration options.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    Following code snippets are implementing `handle_init/2` or `handle_setup/2` callbacks. Those
    are just examples, you can use any [`Membrane.Pipeline` callbacks](https://hexdocs.pm/membrane_core/Membrane.Pipeline.html#callbacks)
    instead.

    ```elixir
    alias Membrane.LiveCompositor

    def handle_init(ctx, opts) do
      spec = [
        ...

        child(:live_compositor, %LiveCompositor{
          framerate: {30, 1},
          server_setup: :start_locally,
        }),

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
  </TabItem>
</Tabs>


### Register input stream `input_1`.

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/input/input_1/register
    Content-Type: application/json

    {
      "type": "rtp_stream",
      "transport_protocol": "tcp_server",
      "port": 9001,
      "video": {
        "decoder": "ffmpeg_h264"
      }
    }
    ```

    After receiving the response you can establish the connection and start sending the stream. Check out [how to deliver input streams](./deliver-input.md) to learn more.

    In this example we are using RTP over TCP, but it could be easily replaced by UDP.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_init(ctx, opts) do
      spec = [
        ...

        input_1_spec
        |> via_in(Pad.ref(:video_input, "input_1"))
        |> get_child(:live_compositor),

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
    where `input_1_spec` is an element that produces H264 video.
  </TabItem>
</Tabs>

### Register input stream `input_2`.

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/input/input_2/register
    Content-Type: application/json

    {
      "type": "rtp_stream",
      "transport_protocol": "tcp_server",
      "port": 9002,
      "video": {
        "decoder": "ffmpeg_h264"
      }
    }
    ```
    After receiving the response you can establish the connection and start sending the stream. Check out [how to deliver input streams](./deliver-input.md) to learn more.

    In this example, we are using RTP over TCP, but it could be easily replaced by UDP.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_init(ctx, opts) do
      spec = [
        ...

        input_2_spec
        |> via_in(Pad.ref(:video_input, "input_2"))
        |> get_child(:live_compositor),

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
    where `input_2_spec` is an element that produces H264 video.
  </TabItem>
</Tabs>

### Register output stream `output_1`.

Configure it to render just an empty [`View`](../api/components/View.md) component with a background color set to `#4d4d4d` (gray).

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/register
    Content-Type: application/json

    {
      "type": "rtp_stream",
      "transport_protocol": "tcp_server",
      "port": 9003,
      "video": {
        "resolution": { "width": 1280, "height": 720 },
        "encoder": {
          "type": "ffmpeg_h264",
          "preset": "ultrafast",
          "initial": {
            "root": {
              "type": "view",
              "background_color_rgba": "#4d4d4dff",
            }
          }
        }
      }
    }
    ```
    You can can configure output framerate with [`LIVE_COMPOSITOR_OUTPUT_FRAMERATE` environmental variable](../deployment/configuration.md#live_compositor_output_framerate).

    After receiving the response you can establish the connection and start listening for the stream. Check out [how to receive output streams](./receive-output.md) to learn more.

    In this example we are using RTP over TCP. If you prefer to use UDP you need to start listening on the specified port before sending the register request to make sure you are not losing first frames.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_init(ctx, opts) do
      spec = [
        ...

        get_child(:live_compositor),
        |> via_out(Pad.ref(:video_output, "output_1"), options: [
          width: 1280,
          height: 720,
          encoder: %LiveCompositor.Encoder.FFmpegH264{
            preset: :ultrafast
          },
          initial: %{
            root: %{
              type: :view,
              background_color_rgba: "#4d4d4dff",
            },
          },
        ])
        |> output_1_spec

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
    where `output_1_spec` is an element that can consume H264 video.
  </TabItem>
</Tabs>

`View` component does not have any children, so on the output you should see just a blank screen of a specified color as shown below.

<div style={{textAlign: 'center'}}>
    <img src={SimpleSceneEmpty} style={{ width: 600 }} />
    Output stream
</div>

## Update scene to show an input

Update output to render a [`View`](../api/components/View.md) component with an [`InputStream`](../api/components/InputStream.md) as its child.

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/update
    Content-Type: application/json

    {
      "video": {
        "root": {
          "type": "view",
          "background_color_rgba": "#4d4d4dff",
          "children": [
            { "type": "input_stream", "input_id": "input_1" },
          ]
        }
      }
    }
    ```
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_setup(ctx, state) do
      request = %LiveCompositor.Request.UpdateVideoOutput{
        output_id: "output_1",
        root: %{
          type: :view,
          background_color_rgba: "#4d4d4dff",
          children: [
            %{ type: :input_stream, input_id: :input_1 }
          ]
        }
      }
      {[notify_child: {:live_compositor, request}], state}
    end
    ```
  </TabItem>
</Tabs>

The input stream in the example has a resolution `1920x1080` and it is rendered on the `1270x720` output. As a result only part of the stream is visible.

<div style={{textAlign: 'center'}}>
    <img src={SimpleSceneOverflow} style={{ width: 600 }} />
    Output stream
</div>

## Resize input stream to fit inside the output

Wrap an [`InputStream`](../api/components/InputStream.md) component with a [`Rescaler`](../api/components/Rescaler.md).

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/update
    Content-Type: application/json

    {
      "video": {
        "root": {
          "type": "view",
          "background_color_rgba": "#4d4d4dff",
          "children": [
            {
              "type": "rescaler",
              "child": { "type": "input_stream", "input_id": "input_1" },
            }
          ]
        }
      }
    }
    ```
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_setup(ctx, state) do
      request = %LiveCompositor.Request.UpdateVideoOutput{
        output_id: "output_1",
        root: %{
          type: :view,
          background_color_rgba: "#4d4d4dff",
          children: [
            %{
              type: :rescaler,
              child: %{ type: :input_stream, input_id: :input_1 },
            }
          ]
        }
      }
      {[notify_child: {:live_compositor, request}], state}
    end
    ```
  </TabItem>
</Tabs>

Input stream now fully fits inside the output.

<div style={{textAlign: 'center'}}>
    <img src={SimpleSceneFitted} style={{ width: 600 }} />
    Output stream
</div>

:::note
The same effect (for single input) could be achieved by either:
- Setting `InputStream` as a root directly. It would only work if aspect ratio of input and output is the same.
- Replacing `View` component with a `Rescaler`.
:::

## Show both inputs side by side

Add another [`InputStream`](../api/components/InputStream.md) wrapped with [`Rescaler`](../api/components/Rescaler.md).

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/update
    Content-Type: application/json

    {
      "video": {
        "root": {
          "type": "view",
          "background_color_rgba": "#4d4d4dff",
          "children": [
            {
              "type": "rescaler",
              "child": { "type": "input_stream", "input_id": "input_1" },
            },
            {
              "type": "rescaler",
              "child": { "type": "input_stream", "input_id": "input_2" },
            }
          ]
        }
      }
    }
    ```
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_setup(ctx, state) do
      request = %LiveCompositor.Request.UpdateVideoOutput{
        output_id: "output_1",
        root: %{
          type: :view,
          background_color_rgba: "#4d4d4dff",
          children: [
            %{
              type: :rescaler,
              child: %{ type: :input_stream, input_id: :input_1 },
            },
            %{
              type: :rescaler,
              child: %{ type: :input_stream, input_id: :input_2 },
            }
          ]
        }
      }
      {[notify_child: {:live_compositor, request}], state}
    end
    ```
  </TabItem>
</Tabs>

By default, a `View` component positions its children next to each other in a row. Each child without a defined width or
height fills available space inside the parent component. To place them in a column, set a `direction: "column"` option.

In an example below we can see that:
- `View` has 2 children components with unspecified dimensions, so they will divide available width exactly in half.
- Each `Rescaler` component has a size `640x720` (half of `1280x720`), but it needs to fit an input stream with `16:9` aspect ratio.

<div style={{textAlign: 'center'}}>
    <img src={SimpleSceneBothInputs} style={{ width: 600 }} />
    Output stream
</div>

## Place one of the inputs in the top right corner

Specify `width` and `height` of one of the `Rescaler` components and position it using `top`/`right` options in the corner.

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/update
    Content-Type: application/json

    {
      "video": {
        "root": {
          "type": "view",
          "background_color_rgba": "#4d4d4dff",
          "children": [
            {
              "type": "rescaler",
              "child": { "type": "input_stream", "input_id": "input_1" },
            },
            {
              "type": "rescaler",
              "width": 320,
              "height": 180,
              "top": 20,
              "right": 20,
              "child": { "type": "input_stream", "input_id": "input_2" },
            }
          ]
        }
      }
    }
    ```
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_setup(ctx, state) do
      request = %LiveCompositor.Request.UpdateVideoOutput{
        output_id: "output_1",
        root: %{
          type: :view,
          background_color_rgba: "#4d4d4dff",
          children: [
            %{
              type: :rescaler,
              child: %{ type: :input_stream, input_id: :input_1 },
            },
            %{
              type: :rescaler,
              width: 320,
              height: 180,
              top: 20,
              right: 20,
              child: %{ type: :input_stream, input_id: :input_2 },
            }
          ]
        }
      }
      {[notify_child: {:live_compositor, request}], state}
    end
    ```
  </TabItem>
</Tabs>

When you specify `top`/`right` options on the `Rescaler` component, the `View` component does not take that component
into account when calculating the row layout of its children. See [absolute positioning](../api/components/View.md#absolute-positioning) to learn more.

As a result:
- The first child extends to the full width of a parent.
- The second component takes the specified size, and it is positioned in the top-right corner (20px from border).

<div style={{textAlign: 'center'}}>
    <img src={SimpleSceneAbsolutePosition} style={{ width: 600 }} />
    Output stream
</div>
