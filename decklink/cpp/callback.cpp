#include "callback.h"
#include "api.h"

ULONG InputCallbackWrapper::AddRef(void) {
  return __sync_add_and_fetch(&refcount, 1);
}

ULONG InputCallbackWrapper::Release(void) {
  int32_t new_refcount = __sync_sub_and_fetch(&refcount, 1);
  if (new_refcount == 0) {
    delete this;
    return 0;
  }
  return new_refcount;
}

HRESULT InputCallbackWrapper::VideoInputFrameArrived(
    IDeckLinkVideoInputFrame *video_frame,
    IDeckLinkAudioInputPacket *audio_packet) {
  auto result = this->cb->video_input_frame_arrived(video_frame, audio_packet);
  return static_cast<HRESULT>(result);
}

HRESULT InputCallbackWrapper::VideoInputFormatChanged(
    BMDVideoInputFormatChangedEvents events, IDeckLinkDisplayMode *display_mode,
    BMDDetectedVideoInputFormatFlags flags) {
  return 0;
}
