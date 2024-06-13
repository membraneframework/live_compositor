#include "decklink/decklink_sdk/include/DeckLinkAPI.h"
#include "decklink/src/enums.rs.h"

REFIID declare_id(DeclarationId id) {
#define CASE(VALUE)                                                            \
  case DeclarationId::VALUE:                                                   \
    return IID_IDeckLink##VALUE;

  switch (id) {
    CASE(VideoOutputCallback)
    CASE(InputCallback)
    CASE(EncoderInputCallback)
    CASE(MemoryAllocator)
    CASE(AudioOutputCallback)
    CASE(Iterator)
    CASE(APIInformation)
    CASE(Output)
    CASE(Input)
    CASE(HDMIInputEDID)
    CASE(EncoderInput)
    CASE(VideoFrame)
    CASE(MutableVideoFrame)
    CASE(VideoFrame3DExtensions)
    CASE(VideoFrameMetadataExtensions)
    CASE(VideoInputFrame)
    CASE(AncillaryPacket)
    CASE(AncillaryPacketIterator)
    CASE(VideoFrameAncillaryPackets)
    CASE(VideoFrameAncillary)
    CASE(EncoderPacket)
    CASE(EncoderVideoPacket)
    CASE(EncoderAudioPacket)
    CASE(H265NALPacket)
    CASE(AudioInputPacket)
    CASE(ScreenPreviewCallback)
    CASE(GLScreenPreviewHelper)
    CASE(NotificationCallback)
    CASE(Notification)
    CASE(ProfileAttributes)
    CASE(ProfileIterator)
    CASE(Profile)
    CASE(ProfileCallback)
    CASE(ProfileManager)
    CASE(Status)
    CASE(Keyer)
    CASE(VideoConversion)
    CASE(DeviceNotificationCallback)
    CASE(Discovery)
  }
#undef CASE
}

BMDDeckLinkAttributeID flag_attribute_id(FlagAttributeId id) {
#define CASE(VALUE)                                                            \
  case FlagAttributeId::VALUE:                                                 \
    return BMDDeckLink##VALUE;

  switch (id) {
    CASE(SupportsInternalKeying)
    CASE(SupportsExternalKeying)
    CASE(SupportsInputFormatDetection)
    CASE(HasReferenceInput)
    CASE(HasSerialPort)
    CASE(HasAnalogVideoOutputGain)
    CASE(CanOnlyAdjustOverallVideoOutputGain)
    CASE(HasVideoInputAntiAliasingFilter)
    CASE(HasBypass)
    CASE(SupportsClockTimingAdjustment)
    CASE(SupportsFullFrameReferenceInputTimingOffset)
    CASE(SupportsSMPTELevelAOutput)
    CASE(SupportsAutoSwitchingPPsFOnInput)
    CASE(SupportsDualLinkSDI)
    CASE(SupportsQuadLinkSDI)
    CASE(SupportsIdleOutput)
    CASE(VANCRequires10BitYUVVideoFrames)
    CASE(HasLTCTimecodeInput)
    CASE(SupportsHDRMetadata)
    CASE(SupportsColorspaceMetadata)
    CASE(SupportsHDMITimecode)
    CASE(SupportsHighFrameRateTimecode)
    CASE(SupportsSynchronizeToCaptureGroup)
    CASE(SupportsSynchronizeToPlaybackGroup)
    CASE(HasMonitorOut)
  }
#undef CASE
}

BMDDeckLinkAttributeID integer_attribute_id(IntegerAttributeId id) {
#define CASE(VALUE)                                                            \
  case IntegerAttributeId::VALUE:                                              \
    return BMDDeckLink##VALUE;

  switch (id) {
    CASE(MaximumAudioChannels)
    CASE(MaximumHDMIAudioChannels)
    CASE(MaximumAnalogAudioInputChannels)
    CASE(MaximumAnalogAudioOutputChannels)
    CASE(NumberOfSubDevices)
    CASE(SubDeviceIndex)
    CASE(PersistentID)
    CASE(DeviceGroupID)
    CASE(TopologicalID)
    CASE(VideoOutputConnections)
    CASE(VideoInputConnections)
    CASE(AudioOutputConnections)
    CASE(AudioInputConnections)
    CASE(VideoIOSupport)
    CASE(DeckControlConnections)
    CASE(DeviceInterface)
    CASE(AudioInputRCAChannelCount)
    CASE(AudioInputXLRChannelCount)
    CASE(AudioOutputRCAChannelCount)
    CASE(AudioOutputXLRChannelCount)
    CASE(ProfileID)
    CASE(Duplex)
    CASE(MinimumPrerollFrames)
    CASE(SupportedDynamicRange)
    CASE(MezzanineType)
  }
#undef CASE
}

BMDDeckLinkAttributeID float_attribute_id(FloatAttributeId id) {
#define CASE(VALUE)                                                            \
  case FloatAttributeId::VALUE:                                                \
    return BMDDeckLink##VALUE;

  switch (id) {
    CASE(VideoInputGainMinimum)
    CASE(VideoInputGainMaximum)
    CASE(VideoOutputGainMinimum)
    CASE(VideoOutputGainMaximum)
    CASE(MicrophoneInputGainMinimum)
    CASE(MicrophoneInputGainMaximum)
  }
#undef CASE
}

BMDDeckLinkAttributeID string_attribute_id(StringAttributeId id) {
#define CASE(VALUE)                                                            \
  case StringAttributeId::VALUE:                                               \
    return BMDDeckLink##VALUE;

  switch (id) {
    CASE(SerialPortDeviceName)
    CASE(VendorName)
    CASE(DisplayName)
    CASE(ModelName)
    CASE(DeviceHandle)
    CASE(EthernetMACAddress)
  }
#undef CASE
}

BMDVideoConnection from_video_connection(VideoConnection conn) {
#define CASE(VALUE)                                                            \
  case VideoConnection::VALUE:                                                 \
    return bmdVideoConnection##VALUE;

  switch (conn) {
    CASE(Unspecified)
    CASE(SDI)
    CASE(HDMI)
    CASE(OpticalSDI)
    CASE(Component)
    CASE(Composite)
    CASE(SVideo)
    CASE(Ethernet)
    CASE(OpticalEthernet)
  }
#undef CASE
}

VideoConnection into_video_connection(BMDVideoConnection conn) {
#define CASE(VALUE)                                                            \
  case bmdVideoConnection##VALUE:                                              \
    return VideoConnection::VALUE;

  switch (conn) {
    CASE(Unspecified)
    CASE(SDI)
    CASE(HDMI)
    CASE(OpticalSDI)
    CASE(Component)
    CASE(Composite)
    CASE(SVideo)
    CASE(Ethernet)
    CASE(OpticalEthernet)
  }
#undef CASE
}

BMDAudioConnection from_audio_connection(AudioConnection conn) {
#define CASE(VALUE)                                                            \
  case AudioConnection::VALUE:                                                 \
    return bmdAudioConnection##VALUE;

  switch (conn) {
    CASE(Embedded)
    CASE(AESEBU)
    CASE(Analog)
    CASE(AnalogXLR)
    CASE(AnalogRCA)
    CASE(Microphone)
    CASE(Headphones)
  }
#undef CASE
}

AudioConnection into_audio_connection(BMDAudioConnection conn) {
#define CASE(VALUE)                                                            \
  case bmdAudioConnection##VALUE:                                              \
    return AudioConnection::VALUE;

  switch (conn) {
    CASE(Embedded)
    CASE(AESEBU)
    CASE(Analog)
    CASE(AnalogXLR)
    CASE(AnalogRCA)
    CASE(Microphone)
    CASE(Headphones)
  }
#undef CASE
}

BMDDisplayMode from_display_mode(DisplayMode mode) {
#define CASE(VALUE)                                                            \
  case DisplayMode::VALUE:                                                     \
    return bmd##VALUE;

  switch (mode) {
    /* SD Modes */
    CASE(ModeNTSC)
    CASE(ModeNTSC2398) // 3:2 pulldown
    CASE(ModePAL)
    CASE(ModeNTSCp)
    CASE(ModePALp)

    /* HD 1080 Modes */
    CASE(ModeHD1080p2398)
    CASE(ModeHD1080p24)
    CASE(ModeHD1080p25)
    CASE(ModeHD1080p2997)
    CASE(ModeHD1080p30)
    CASE(ModeHD1080p4795)
    CASE(ModeHD1080p48)
    CASE(ModeHD1080p50)
    CASE(ModeHD1080p5994)
    CASE(ModeHD1080p6000) // N.B. This _really_ is 60.00 Hz.
    CASE(ModeHD1080p9590)
    CASE(ModeHD1080p96)
    CASE(ModeHD1080p100)
    CASE(ModeHD1080p11988)
    CASE(ModeHD1080p120)
    CASE(ModeHD1080i50)
    CASE(ModeHD1080i5994)
    CASE(ModeHD1080i6000) // N.B. This _really_ is 60.00 Hz.

    /* HD 720 Modes */
    CASE(ModeHD720p50)
    CASE(ModeHD720p5994)
    CASE(ModeHD720p60)

    /* 2K Modes */
    CASE(Mode2k2398)
    CASE(Mode2k24)
    CASE(Mode2k25)

    /* 2K DCI Modes */
    CASE(Mode2kDCI2398)
    CASE(Mode2kDCI24)
    CASE(Mode2kDCI25)
    CASE(Mode2kDCI2997)
    CASE(Mode2kDCI30)
    CASE(Mode2kDCI4795)
    CASE(Mode2kDCI48)
    CASE(Mode2kDCI50)
    CASE(Mode2kDCI5994)
    CASE(Mode2kDCI60)
    CASE(Mode2kDCI9590)
    CASE(Mode2kDCI96)
    CASE(Mode2kDCI100)
    CASE(Mode2kDCI11988)
    CASE(Mode2kDCI120)

    /* 4K UHD Modes */
    CASE(Mode4K2160p2398)
    CASE(Mode4K2160p24)
    CASE(Mode4K2160p25)
    CASE(Mode4K2160p2997)
    CASE(Mode4K2160p30)
    CASE(Mode4K2160p4795)
    CASE(Mode4K2160p48)
    CASE(Mode4K2160p50)
    CASE(Mode4K2160p5994)
    CASE(Mode4K2160p60)
    CASE(Mode4K2160p9590)
    CASE(Mode4K2160p96)
    CASE(Mode4K2160p100)
    CASE(Mode4K2160p11988)
    CASE(Mode4K2160p120)

    /* 4K DCI Modes */
    CASE(Mode4kDCI2398)
    CASE(Mode4kDCI24)
    CASE(Mode4kDCI25)
    CASE(Mode4kDCI2997)
    CASE(Mode4kDCI30)
    CASE(Mode4kDCI4795)
    CASE(Mode4kDCI48)
    CASE(Mode4kDCI50)
    CASE(Mode4kDCI5994)
    CASE(Mode4kDCI60)
    CASE(Mode4kDCI9590)
    CASE(Mode4kDCI96)
    CASE(Mode4kDCI100)
    CASE(Mode4kDCI11988)
    CASE(Mode4kDCI120)

    /* 8K UHD Modes */
    CASE(Mode8K4320p2398)
    CASE(Mode8K4320p24)
    CASE(Mode8K4320p25)
    CASE(Mode8K4320p2997)
    CASE(Mode8K4320p30)
    CASE(Mode8K4320p4795)
    CASE(Mode8K4320p48)
    CASE(Mode8K4320p50)
    CASE(Mode8K4320p5994)
    CASE(Mode8K4320p60)

    /* 8K DCI Modes */
    CASE(Mode8kDCI2398)
    CASE(Mode8kDCI24)
    CASE(Mode8kDCI25)
    CASE(Mode8kDCI2997)
    CASE(Mode8kDCI30)
    CASE(Mode8kDCI4795)
    CASE(Mode8kDCI48)
    CASE(Mode8kDCI50)
    CASE(Mode8kDCI5994)
    CASE(Mode8kDCI60)

    /* PC Modes */
    CASE(Mode640x480p60)
    CASE(Mode800x600p60)
    CASE(Mode1440x900p50)
    CASE(Mode1440x900p60)
    CASE(Mode1440x1080p50)
    CASE(Mode1440x1080p60)
    CASE(Mode1600x1200p50)
    CASE(Mode1600x1200p60)
    CASE(Mode1920x1200p50)
    CASE(Mode1920x1200p60)
    CASE(Mode1920x1440p50)
    CASE(Mode1920x1440p60)
    CASE(Mode2560x1440p50)
    CASE(Mode2560x1440p60)
    CASE(Mode2560x1600p50)
    CASE(Mode2560x1600p60)

    /* Special Modes */
    CASE(ModeUnknown)
  }
#undef CASE
}

DisplayMode into_display_mode(BMDDisplayMode mode) {
#define CASE(VALUE)                                                            \
  case bmd##VALUE:                                                             \
    return DisplayMode::VALUE;

  switch (mode) {
    /* SD Modes */
    CASE(ModeNTSC)
    CASE(ModeNTSC2398) // 3:2 pulldown
    CASE(ModePAL)
    CASE(ModeNTSCp)
    CASE(ModePALp)

    /* HD 1080 Modes */
    CASE(ModeHD1080p2398)
    CASE(ModeHD1080p24)
    CASE(ModeHD1080p25)
    CASE(ModeHD1080p2997)
    CASE(ModeHD1080p30)
    CASE(ModeHD1080p4795)
    CASE(ModeHD1080p48)
    CASE(ModeHD1080p50)
    CASE(ModeHD1080p5994)
    CASE(ModeHD1080p6000) // N.B. This _really_ is 60.00 Hz.
    CASE(ModeHD1080p9590)
    CASE(ModeHD1080p96)
    CASE(ModeHD1080p100)
    CASE(ModeHD1080p11988)
    CASE(ModeHD1080p120)
    CASE(ModeHD1080i50)
    CASE(ModeHD1080i5994)
    CASE(ModeHD1080i6000) // N.B. This _really_ is 60.00 Hz.

    /* HD 720 Modes */
    CASE(ModeHD720p50)
    CASE(ModeHD720p5994)
    CASE(ModeHD720p60)

    /* 2K Modes */
    CASE(Mode2k2398)
    CASE(Mode2k24)
    CASE(Mode2k25)

    /* 2K DCI Modes */
    CASE(Mode2kDCI2398)
    CASE(Mode2kDCI24)
    CASE(Mode2kDCI25)
    CASE(Mode2kDCI2997)
    CASE(Mode2kDCI30)
    CASE(Mode2kDCI4795)
    CASE(Mode2kDCI48)
    CASE(Mode2kDCI50)
    CASE(Mode2kDCI5994)
    CASE(Mode2kDCI60)
    CASE(Mode2kDCI9590)
    CASE(Mode2kDCI96)
    CASE(Mode2kDCI100)
    CASE(Mode2kDCI11988)
    CASE(Mode2kDCI120)

    /* 4K UHD Modes */
    CASE(Mode4K2160p2398)
    CASE(Mode4K2160p24)
    CASE(Mode4K2160p25)
    CASE(Mode4K2160p2997)
    CASE(Mode4K2160p30)
    CASE(Mode4K2160p4795)
    CASE(Mode4K2160p48)
    CASE(Mode4K2160p50)
    CASE(Mode4K2160p5994)
    CASE(Mode4K2160p60)
    CASE(Mode4K2160p9590)
    CASE(Mode4K2160p96)
    CASE(Mode4K2160p100)
    CASE(Mode4K2160p11988)
    CASE(Mode4K2160p120)

    /* 4K DCI Modes */
    CASE(Mode4kDCI2398)
    CASE(Mode4kDCI24)
    CASE(Mode4kDCI25)
    CASE(Mode4kDCI2997)
    CASE(Mode4kDCI30)
    CASE(Mode4kDCI4795)
    CASE(Mode4kDCI48)
    CASE(Mode4kDCI50)
    CASE(Mode4kDCI5994)
    CASE(Mode4kDCI60)
    CASE(Mode4kDCI9590)
    CASE(Mode4kDCI96)
    CASE(Mode4kDCI100)
    CASE(Mode4kDCI11988)
    CASE(Mode4kDCI120)

    /* 8K UHD Modes */
    CASE(Mode8K4320p2398)
    CASE(Mode8K4320p24)
    CASE(Mode8K4320p25)
    CASE(Mode8K4320p2997)
    CASE(Mode8K4320p30)
    CASE(Mode8K4320p4795)
    CASE(Mode8K4320p48)
    CASE(Mode8K4320p50)
    CASE(Mode8K4320p5994)
    CASE(Mode8K4320p60)

    /* 8K DCI Modes */
    CASE(Mode8kDCI2398)
    CASE(Mode8kDCI24)
    CASE(Mode8kDCI25)
    CASE(Mode8kDCI2997)
    CASE(Mode8kDCI30)
    CASE(Mode8kDCI4795)
    CASE(Mode8kDCI48)
    CASE(Mode8kDCI50)
    CASE(Mode8kDCI5994)
    CASE(Mode8kDCI60)

    /* PC Modes */
    CASE(Mode640x480p60)
    CASE(Mode800x600p60)
    CASE(Mode1440x900p50)
    CASE(Mode1440x900p60)
    CASE(Mode1440x1080p50)
    CASE(Mode1440x1080p60)
    CASE(Mode1600x1200p50)
    CASE(Mode1600x1200p60)
    CASE(Mode1920x1200p50)
    CASE(Mode1920x1200p60)
    CASE(Mode1920x1440p50)
    CASE(Mode1920x1440p60)
    CASE(Mode2560x1440p50)
    CASE(Mode2560x1440p60)
    CASE(Mode2560x1600p50)
    CASE(Mode2560x1600p60)

    /* Special Modes */
    CASE(ModeUnknown)
  }
#undef CASE
}

BMDPixelFormat from_pixel_format(PixelFormat format) {
#define CASE(VALUE)                                                            \
  case PixelFormat::VALUE:                                                     \
    return bmd##VALUE;

  switch (format) {
    CASE(FormatUnspecified)
    CASE(Format8BitYUV)
    CASE(Format10BitYUV)
    CASE(Format10BitYUVA)
    CASE(Format8BitARGB)
    CASE(Format8BitBGRA)
    CASE(Format10BitRGB)
    CASE(Format12BitRGB)
    CASE(Format12BitRGBLE)
    CASE(Format10BitRGBXLE)
    CASE(Format10BitRGBX)
    CASE(FormatH265)
    CASE(FormatDNxHR)
  }
#undef CASE
}
PixelFormat into_pixel_format(BMDPixelFormat format) {
#define CASE(VALUE)                                                            \
  case bmd##VALUE:                                                             \
    return PixelFormat::VALUE;

  switch (format) {
    CASE(FormatUnspecified)
    CASE(Format8BitYUV)
    CASE(Format10BitYUV)
    CASE(Format10BitYUVA)
    CASE(Format8BitARGB)
    CASE(Format8BitBGRA)
    CASE(Format10BitRGB)
    CASE(Format12BitRGB)
    CASE(Format12BitRGBLE)
    CASE(Format10BitRGBXLE)
    CASE(Format10BitRGBX)
    CASE(FormatH265)
    CASE(FormatDNxHR)
  }
#undef CASE
}

BMDVideoInputConversionMode
from_video_input_conversion_mode(VideoInputConversionMode mode) {
#define CASE(VALUE)                                                            \
  case VideoInputConversionMode::VALUE:                                        \
    return bmd##VALUE;

  switch (mode) {
    CASE(NoVideoInputConversion)
    CASE(VideoInputLetterboxDownconversionFromHD1080)
    CASE(VideoInputAnamorphicDownconversionFromHD1080)
    CASE(VideoInputLetterboxDownconversionFromHD720)
    CASE(VideoInputAnamorphicDownconversionFromHD720)
    CASE(VideoInputLetterboxUpconversion)
    CASE(VideoInputAnamorphicUpconversion)
  }
#undef CASE
}

VideoInputConversionMode
into_video_input_conversion_mode(BMDVideoInputConversionMode mode) {
#define CASE(VALUE)                                                            \
  case bmd##VALUE:                                                             \
    return VideoInputConversionMode::VALUE;

  switch (mode) {
    CASE(NoVideoInputConversion)
    CASE(VideoInputLetterboxDownconversionFromHD1080)
    CASE(VideoInputAnamorphicDownconversionFromHD1080)
    CASE(VideoInputLetterboxDownconversionFromHD720)
    CASE(VideoInputAnamorphicDownconversionFromHD720)
    CASE(VideoInputLetterboxUpconversion)
    CASE(VideoInputAnamorphicUpconversion)
  }
#undef CASE
}

BMDSupportedVideoModeFlags
from_supported_video_mode_flags(SupportedVideoModeFlags flags) {
  BMDSupportedVideoModeFlags bmd_flags = bmdSupportedVideoModeDefault;
  if (flags.supports_keying) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeKeying;
  }
  if (flags.supports_dual_stream_3d) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeDualStream3D;
  }
  if (flags.supports_SDI_single_link) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeSDISingleLink;
  }
  if (flags.supports_SDI_dual_link) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeSDIDualLink;
  }
  if (flags.supports_SDI_quad_link) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeSDIQuadLink;
  }
  if (flags.supports_in_any_profile) {
    bmd_flags = bmd_flags | bmdSupportedVideoModeInAnyProfile;
  }
  if (flags.supports_PsF) {
    bmd_flags = bmd_flags | bmdSupportedVideoModePsF;
  }

  return bmd_flags;
}

SupportedVideoModeFlags
into_supported_video_mode_flags(BMDSupportedVideoModeFlags bmd_flags) {
  SupportedVideoModeFlags flags;
  flags.supports_keying = (bmd_flags & bmdSupportedVideoModeKeying) != 0;
  flags.supports_dual_stream_3d =
      (bmd_flags & bmdSupportedVideoModeDualStream3D) != 0;
  flags.supports_SDI_single_link =
      (bmd_flags & bmdSupportedVideoModeSDISingleLink) != 0;
  flags.supports_SDI_dual_link =
      (bmd_flags & bmdSupportedVideoModeSDIDualLink) != 0;
  flags.supports_SDI_quad_link =
      (bmd_flags & bmdSupportedVideoModeSDIQuadLink) != 0;
  flags.supports_in_any_profile =
      (bmd_flags & bmdSupportedVideoModeInAnyProfile) != 0;
  flags.supports_PsF = (bmd_flags & bmdSupportedVideoModePsF) != 0;
  return flags;
}

BMDVideoInputFlags from_video_input_flags(VideoInputFlags flags) {
  BMDVideoInputFlags bmd_flags = bmdVideoInputFlagDefault;
  if (flags.enable_format_detection) {
    bmd_flags = bmd_flags | bmdVideoInputEnableFormatDetection;
  }
  if (flags.dual_stream_3d) {
    bmd_flags = bmd_flags | bmdVideoInputDualStream3D;
  }
  if (flags.synchronize_to_capture_group) {
    bmd_flags = bmd_flags | bmdVideoInputSynchronizeToCaptureGroup;
  }
  return bmd_flags;
}

VideoInputFlags into_video_input_flags(BMDVideoInputFlags bmd_flags) {
  VideoInputFlags flags;
  flags.enable_format_detection =
      (bmd_flags & bmdVideoInputEnableFormatDetection) != 0;
  flags.dual_stream_3d = (bmd_flags & bmdVideoInputDualStream3D) != 0;
  flags.synchronize_to_capture_group =
      (bmd_flags & bmdVideoInputSynchronizeToCaptureGroup) != 0;
  return flags;
}
