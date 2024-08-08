---
description: WebSocket events
---

# Events

LiveCompositor is using WebSocket connection to send events to the connected clients. Supported events are listed below.

### `VIDEO_INPUT_DELIVERED`

```typescript
type Event = {
  type: "VIDEO_INPUT_DELIVERED";
  input_id: string;
}
```

The compositor received the input, and the first frames of that input are ready to be used. If you want to ensure that some inputs are ready before you send the [`start`](./routes.md#start-request) request, you can wait for those events for specific inputs.

### `VIDEO_INPUT_PLAYING`

```typescript
type Event = {
  type: "VIDEO_INPUT_PLAYING";
  input_id: string;
}
```

The compositor received the input and is using the first frame for rendering. This event will not be sent before the [`start`](./routes.md#start-request) request.

This event is usually sent at the same time as `VIDEO_INPUT_DELIVERED` except for 2 cases:
- Before [`start`](./routes.md#start-request) request.
- If input has the `offset_ms` field defined.

### `VIDEO_INPUT_EOS`

```typescript
type Event = {
  type: "VIDEO_INPUT_EOS";
  input_id: string;
}
```

The input stream has ended and all the frames were already processed.
It's not emitted on [`input unregister`](routes.md#unregister-input).

### `AUDIO_INPUT_DELIVERED`

```typescript
type Event = {
  type: "AUDIO_INPUT_DELIVERED";
  input_id: string;
}
```

The compositor received the input, and the first samples on that input are ready to be used. If you want to ensure that some inputs are ready before you send the [`start`](./routes.md#start-request) request, you can wait for those events for specific inputs.

### `AUDIO_INPUT_PLAYING`

```typescript
type Event = {
  type: "AUDIO_INPUT_PLAYING";
  input_id: string;
}
```

The compositor received the input and is using the first samples for rendering. This event will not be sent before the [`start`](./routes.md#start-request) request.

This event is usually sent at the same time as `AUDIO_INPUT_DELIVERED` except for 2 cases:
- Before [`start`](./routes.md#start-request) request.
- If input has the `offset_ms` field defined.

### `AUDIO_INPUT_EOS`

```typescript
type Event = {
  type: "AUDIO_INPUT_EOS";
  input_id: string;
}
```

The input stream has ended and all the audio samples were already processed.
It's not emitted on [`input unregister`](routes.md#unregister-input).

### `OUTPUT_EOS`

```typescript
type Event = {
  type: "OUTPUT_EOS",
  output_id: string
}
```

The output has ended. All video frames and audio samples were sent/written.
