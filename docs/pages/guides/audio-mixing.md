import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Audio mixing

This guide will explain how to mix audio.

## Configure input and output

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

### Register input `input_1`

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/input/input_1/register
    Content-Type: application/json

    {
      "type": "rtp_stream",
      "transport_protocol": "tcp_server",
      "port": 9001,
      "audio": {
        "decoder": "opus"
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

        input_1_spec
        |> via_in(Pad.ref(:audio_input, "input_1"))
        |> get_child(:live_compositor),

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
    where `input_1_spec` is an element that produces Opus audio.
  </TabItem>
</Tabs>

:::note
<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    You can configure both `audio` and `video` in the same `input`.
  </TabItem>
    <TabItem value="membrane" label="Membrane Framework">
    `video_input` and `audio_input` are separate pads.
  </TabItem>
</Tabs>
:::

### Register input `input_2`

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/input/input_2/register
    Content-Type: application/json

    {
      "type": "mp4",
      "url": "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4", 
    }
    ```

    When using MP4 as an input, video and audio parameters are automatically detected.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_init(ctx, opts) do
      spec = [
        ...

        input_2_spec
        |> via_in(Pad.ref(:audio_input, "input_2"))
        |> get_child(:live_compositor),

        ...
      ]
      {[spec: spec], %{}}
    end
    ```
    where `input_2_spec` is an element that produces Opus audio.

    Using Membrane, video and audio inputs should be a separate pads.
  </TabItem>
</Tabs>

### Register output `output_1`

Configure it to mix audio from `input_1` and audio from `input_2` with reduced volume to 90% of the original volume. 

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/register
    Content-Type: application/json

    {
      "type": "rtp_stream",
      "transport_protocol": "tcp_server",
      "port": 9003,
      "audio": {
        "initial": {
            "inputs": [
                { "input_id": "input_1" },
                { "input_id": "input_2", "volume": 0.9 }
            ]
        }
      }
    }
    ```
    You can configure output sample rate with the [`LIVE_COMPOSITOR_OUTPUT_SAMPLE_RATE` environmental variable](../deployment/configuration.md#live_compositor_output_sample_rate).

    After receiving the response you can establish the connection and start listening for the stream. Check out [how to receive output streams](./receive-output.md) to learn more.

    In this example we are using RTP over TCP. If you prefer to use UDP you need to start listening on the specified port before sending the register request to make sure you are not losing first frames.
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_init(ctx, opts) do
      spec = [
        ...

        get_child(:live_compositor),
        |> via_out(Pad.ref(:audio_output, "output_1"), options: [
          encoder: %LiveCompositor.Encoder.Opus{
            channels: :stereo
          },
          initial: %{
            %{input_id: "input_1"},
            %{input_id: "input_2", volume: 0.9}
          }
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

:::note
<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    You can configure both `audio` and `video` in the same `output`.
  </TabItem>
    <TabItem value="membrane" label="Membrane Framework">
    `video_output` and `audio_output` are separate pads.
  </TabItem>
</Tabs>
:::

### Update output `output_1`

Send only audio from `input_1` to output.

<Tabs queryString="lang">
  <TabItem value="http" label="HTTP">
    ```http
    POST: /api/output/output_1/update
    Content-Type: application/json

    {
      "audio": {
        "inputs": [
          { "input_id": "input_1" }
        ]
      }
    }
    ```
  </TabItem>
  <TabItem value="membrane" label="Membrane Framework">
    ```elixir
    def handle_setup(ctx, state) do
      request = %LiveCompositor.Request.UpdateAudioOutput{
        output_id: "output_1",
        inputs: [
          %{input_id: "input_1"}
        ]
      }
      {[notify_child: {:live_compositor, request}], state}
    end
    ```
  </TabItem>
</Tabs>
