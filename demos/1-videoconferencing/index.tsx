import React, { useState, useEffect } from 'react';
import { gstStreamWebcam } from '../utils/gst';
import { downloadAsync } from '../utils/utils';
import { ffmpegSendVideoFromMp4, ffplayStartPlayerAsync } from '../utils/ffmpeg';
import path from 'path';
import LiveCompositor, { InputStream, Rescaler, Tiles, View, Text, Image } from 'live-compositor';

const OUTPUT_RESOLUTION = {
  width: 1920,
  height: 1080,
};

const INPUT_PORT = 8004;
const OUTPUT_PORT = 8002;
const IP = '127.0.0.1';
const DISPLAY_LOGS = true;

const BACKGROUND_URL =
  'https://raw.githubusercontent.com/membraneframework-labs/video_compositor_snapshot_tests/main/demo_assets/triangles_background.png';
const CALL_URL =
  'https://raw.githubusercontent.com/membraneframework-labs/video_compositor_snapshot_tests/main/demo_assets/call.mp4';

const INPUT_COUNT_CHANGES = [
  2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1,
];

function App() {
  const [phase, setPhase] = useState(0);

  useEffect(() => {
    const interval = setTimeout(() => {
      setPhase((phase + 1) % INPUT_COUNT_CHANGES.length);
    }, 5000);
    return () => {
      clearTimeout(interval);
    };
  }, [phase]);

  return (
    <View>
      <Rescaler top={0} left={0}>
        <Image imageId="background" />
      </Rescaler>
      <Tiles
        id="tiles"
        padding={5}
        transition={{
          durationMs: 700,
          easingFunction: {
            functionName: 'cubic-bezier',
            points: [0.35, 0.22, 0.1, 0.8],
          },
        }}>
        {[...Array(INPUT_COUNT_CHANGES[phase])].map((_, index) => (
          <StreamTile index={index} />
        ))}
      </Tiles>
    </View>
  );
}

function StreamTile({ index }: { index: number }) {
  return (
    <View>
      <Rescaler>
        <InputStream inputId="input_1" />
      </Rescaler>
      <View height={50} bottom={0} left={0}>
        <View />
        <Text
          fontSize={25}
          align="center"
          colorRgba="#FFFFFFFF"
          backgroundColorRgba="#FF0000FF"
          fontFamily="Arial">
          InputStream {index} 🚀
        </Text>
        <View />
      </View>
    </View>
  );
}

async function run() {
  const useWebCam = process.env.LIVE_COMPOSITOR_WEBCAM !== 'false';
  ffplayStartPlayerAsync(IP, DISPLAY_LOGS, OUTPUT_PORT);

  console.log('start');
  const compositor = await LiveCompositor.create();

  await compositor.registerInput('input_1', {
    type: 'rtp_stream',
    transport_protocol: useWebCam ? 'tcp_server' : 'udp',
    port: INPUT_PORT,
    video: {
      decoder: 'ffmpeg_h264',
    },
  });

  await compositor.registerImage('background', {
    asset_type: 'png',
    url: BACKGROUND_URL,
  });

  await compositor.registerOutput('output_1', {
    type: 'rtp_stream',
    ip: IP,
    port: OUTPUT_PORT,
    video: {
      resolution: OUTPUT_RESOLUTION,
      encoder: {
        type: 'ffmpeg_h264',
        preset: 'fast',
      },
      root: <App />,
    },
  });

  if (useWebCam) {
    gstStreamWebcam(IP, INPUT_PORT, DISPLAY_LOGS);
  } else {
    const callPath = path.join(__dirname, '../assets/call.mp4');
    await downloadAsync(CALL_URL, callPath);
    ffmpegSendVideoFromMp4(INPUT_PORT, callPath, DISPLAY_LOGS);
  }

  await compositor.start();
}
run();
