# How to receive output streams

Live Compositor currently supports 2 output types:
- RTP (H264 + Opus)
- MP4 (H264 + AAC)

MP4 is useful for non real-time rendering scenarios, but for most streaming use cases RTP protocol will be a primary choice.

To deliver RTP output stream in some other format you can use tools like FFmpeg, GStreamer or Membrane Framework to convert between RTP and the desired format.

### RTP over TCP vs UDP

RTP streams can be delivered over TCP or UDP. Depending on your use case, a different choice might make more sense, but in general, we recommend using TCP if possible.

What to choose?
- If you are using the Membrane Framework plugin all communication already happens over TCP. Currently, we do not support any way to configure it.
- Some of the popular multimedia tools do not support RTP over TCP e.g. FFmpeg.
- UDP should only be used for communication on localhost. We do not support retransmission or packet reordering, so if you use it in an unreliable network it might lead to unexpected behavior.
- UDP does not have a congestion control, so if you are using any non-real-time sources for inputs (e.g. streaming file with FFmpeg over RTP) then if you don't throttle the input it might lead to high memory usage.

### What to use to receive RTP streams?

#### Membrane Framework

If you are using the Membrane Framework plugin you do not need anything else. Just connect appropriate output pads to the `LiveCompositor` bin.

#### FFmpeg

FFmpeg does not support RTP over TCP, so you are limited to UDP only.

Start by creating one of the following SDP files:

For streaming H264 video to `127.0.0.1:9001`

**output.sdp**
```
v=0
o=- 0 0 IN IP4 127.0.0.1
s=No Name
c=IN IP4 127.0.0.1
m=video 9001 RTP/AVP 96
a=rtpmap:96 H264/90000
a=fmtp:96 packetization-mode=1
a=rtcp-mux
```

For streaming H264 video to `127.0.0.1:9001` and Opus audio to `127.0.0.1:9002` (multiplexing on the same port does not seem to work).

**output.sdp**
```
v=0
o=- 0 0 IN IP4 127.0.0.1
s=No Name
c=IN IP4 127.0.0.1
m=video 9001 RTP/AVP 96
a=rtpmap:96 H264/90000
a=fmtp:96 packetization-mode=1
a=rtcp-mux
m=audio 9002 RTP/AVP 97
a=rtpmap:97 opus/48000/2
```

To play the stream with `ffplay` run:

```bash
ffplay -protocol_whitelist "file,rtp,udp" output.sdp
```

To save stream as mp4:

```bash
ffmpeg -protocol_whitelist "file,rtp,udp" -i output.sdp out.mp4
```

#### GStreamer

Receive RTP stream on port `127.0.0.1:9001`. Play both audio and video streams using `autovideosink`
and `autoaudiosink`.

Connecting over TCP:
```bash
gst-launch-1.0 rtpptdemux name=demux \
    tcpclientsrc host=127.0.0.1 port=9001 ! \"application/x-rtp-stream\" ! rtpstreamdepay ! queue ! demux. \
    demux.src_96 ! \"application/x-rtp,media=video,clock-rate=90000,encoding-name=H264\" ! queue \
    ! rtph264depay ! decodebin ! videoconvert ! autovideosink \
    demux.src_97 ! \"application/x-rtp,media=audio,clock-rate=48000,encoding-name=OPUS\" ! queue \
    ! rtpopusdepay ! decodebin ! audioconvert ! autoaudiosink
```

Using UDP:
```bash
gst-launch-1.0 rtpptdemux name=demux \
    udpsrc port=9001 ! \"application/x-rtp\" ! queue ! demux. \
    demux.src_96 ! \"application/x-rtp,media=video,clock-rate=90000,encoding-name=H264\" ! queue \
    ! rtph264depay ! decodebin ! videoconvert ! autovideosink \
    demux.src_97 ! \"application/x-rtp,media=audio,clock-rate=48000,encoding-name=OPUS\" ! queue \
    ! rtpopusdepay ! decodebin ! audioconvert ! autoaudiosink sync=false
```
