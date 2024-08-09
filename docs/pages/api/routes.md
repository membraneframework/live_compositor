---
description: API routes to configure the compositor.
---

# Routes

API is served by default on the port 8081. Different port can be configured using [`LIVE_COMPOSITOR_API_PORT`](../deployment/configuration#live_compositor_api_port) environment variable.

## Start request

```http
POST: /api/start
```

```typescript
type RequestBody = {}
```

Starts the processing pipeline. If outputs are registered and defined in the scene then the compositor will start to send the RTP streams.

***

## Outputs configuration

### Register output

```http
POST: /api/output/:output_id/register
```

```typescript
type RequestBody = {
  type: "rtp_stream" | "mp4"
  ... // output specific options
}
```

Register external destination that can be used as a compositor output. See outputs documentation to learn more.

- [RTP](./outputs/rtp.md)
- [MP4](./outputs/mp4.md)

### Unregister output

```http
POST /api/output/:output_id/unregister
```

```typescript
type RequestBody = {
  schedule_time_ms?: number;
}
```

Unregister a previously registered output with an id `:output_id`. 

- `schedule_time_ms` - Time in milliseconds when this request should be applied. Value `0` represents time of [the start request](#start-request).

### Update output

```http
POST: /api/output/:output_id/update
```

```typescript
type RequestBody = {
  video?: {
    root: Component
  };
  audio?: {
    inputs: AudioInput[];
  };
  schedule_time_ms?: number;
}

type AudioInput = {
  input_id: InputId;
  volume?: number;
}
```

Update scene definition and audio mixer configuration for output with ID `:output_id`. The output stream has to be registered first. See [`register output`](./routes.md#register-output) request.

- `video` - Configuration for video output. 
- `video.root` - Root of a component tree/scene that should be rendered for the output. [Learn more](../concept/component)
- `audio` - Configuration for audio output.
- `audio.inputs` - Input streams that should be mixed together and their configuration.
- `audio.inputs[].input_id` - Input ID.
- `audio.inputs[].volume` - (**default=`1.0`**) Float in `[0, 1]` range representing volume.
- `schedule_time_ms` - Time in milliseconds when this request should be applied. Value `0` represents time of [the start request](#start-request).

***

### Request keyframe

```http
POST: /api/output/:output_id/request_keyframe
```

```typescript
type RequestBody = {}
```

Requests additional keyframe (I frame) on the video output.

## Inputs configuration

### Register input

```http
POST: /api/input/:input_id/register
```

```typescript
type RequestBody = {
  type: "rtp_stream" | "mp4" | "decklink";
  ... // input specific options
}
```

Register external source that can be used as a compositor input. See inputs documentation to learn more.

- [RTP](./inputs/rtp.md)
- [MP4](./inputs/mp4.md)
- [DeckLink](./inputs/decklink.md)

### Unregister input

```http
POST: /api/input/:input_id/unregister
```

```typescript
type RequestBody = {
  schedule_time_ms?: number;
}
```

Unregister a previously registered input with an id `:input_id`. 

- `schedule_time_ms` - Time in milliseconds when this request should be applied. Value `0` represents time of [the start request](#start-request).

***

## Renderers configuration

### Register image

```http
POST: /api/image/:image_id/register
```

Register an image asset. Request body is defined in the [image](./renderers/image.md) docs.

### Unregister image

```http
POST: /api/image/:image_id/unregister
```

```typescript
type RequestBody = {}
```

Unregister a previously registered image asset with an id `:image_id`. 

### Register shader

```http
POST: /api/shader/:shader_id/register
```

Register a shader. Request body is defined in the [shader](./renderers/shader.md) docs.

### Unregister shader

```http
POST: /api/shader/:shader_id/unregister
```

```typescript
type RequestBody = {}
```

Unregister a previously registered shader with an id `:shader_id`. 

### Register web renderer instance

```http
POST: /api/web-renderer/:instance_id/register
```

Register a web renderer instance. Request body is defined in the [web renderer](./renderers/web.md) docs.

### Unregister web renderer instance

```http
POST: /api/web-renderer/:instance_id/unregister
```

```typescript
type RequestBody = {}
```

Unregister a previously registered web renderer instance with an id `:instance_id`. 

## Status endpoint 

```http
GET: /status
```

```typescript
type Response = {
  instance_id: string
}
```

Status/health check endpoint. Returns `200 OK`.

- `instance_id` - ID that can be provided using `LIVE_COMPOSITOR_INSTANCE_ID` environment variable. Defaults to random value in the format `live_compositor_{RANDOM_VALUE}`.

## WebSocket endpoint 

```http
/ws
```

Establish WebSocket connection to listen for LiveCompositor events. List of supported events and their descriptions can be found [here](./events.md).


