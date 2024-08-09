import Docs from "@site/pages/api/generated/renderer-Mp4Input.md"

# MP4
An input type that allows the compositor to read static MP4 files.

Mp4 files can contain video and audio tracks encoded with various codecs.
This input type supports mp4 video tracks encoded with h264 and audio tracks encoded with AAC.

If the file contains multiple video or audio tracks, the first audio track and the first video track will be used and the other ones will be ignored.

<Docs />
