// #define (\w+)\s*\(\(([\w\d]+)\)\s*([A-F\dx]+)\)
// pub const $1: $2 = $3;

use super::audio_ffi::{
    SLAint64, SLboolean, SLchar, SLint32, SLmilliHertz, SLmillibel, SLmillimeter, SLuint16,
    SLuint32,
};

pub const KHRONOS_TITLE: &'static [u8; 13usize] = b"KhronosTitle\0";
pub const KHRONOS_ALBUM: &'static [u8; 13usize] = b"KhronosAlbum\0";
pub const KHRONOS_TRACK_NUMBER: &'static [u8; 19usize] = b"KhronosTrackNumber\0";
pub const KHRONOS_ARTIST: &'static [u8; 14usize] = b"KhronosArtist\0";
pub const KHRONOS_GENRE: &'static [u8; 13usize] = b"KhronosGenre\0";
pub const KHRONOS_YEAR: &'static [u8; 12usize] = b"KhronosYear\0";
pub const KHRONOS_COMMENT: &'static [u8; 15usize] = b"KhronosComment\0";
pub const KHRONOS_ARTIST_URL: &'static [u8; 17usize] = b"KhronosArtistURL\0";
pub const KHRONOS_CONTENT_URL: &'static [u8; 18usize] = b"KhronosContentURL\0";
pub const KHRONOS_RATING: &'static [u8; 14usize] = b"KhronosRating\0";
pub const KHRONOS_ALBUM_ART: &'static [u8; 16usize] = b"KhronosAlbumArt\0";
pub const KHRONOS_COPYRIGHT: &'static [u8; 17usize] = b"KhronosCopyright\0";

pub const SL_NODE_PARENT: u32 = 4294967295;
pub const ANDROID_KEY_PCMFORMAT_NUMCHANNELS: &'static [u8; 28usize] =
    b"AndroidPcmFormatNumChannels\0";
pub const ANDROID_KEY_PCMFORMAT_SAMPLERATE: &'static [u8; 27usize] =
    b"AndroidPcmFormatSampleRate\0";
pub const ANDROID_KEY_PCMFORMAT_BITSPERSAMPLE: &'static [u8; 30usize] =
    b"AndroidPcmFormatBitsPerSample\0";
pub const ANDROID_KEY_PCMFORMAT_CONTAINERSIZE: &'static [u8; 30usize] =
    b"AndroidPcmFormatContainerSize\0";
pub const ANDROID_KEY_PCMFORMAT_CHANNELMASK: &'static [u8; 28usize] =
    b"AndroidPcmFormatChannelMask\0";
pub const ANDROID_KEY_PCMFORMAT_ENDIANNESS: &'static [u8; 27usize] =
    b"AndroidPcmFormatEndianness\0";
pub const __GNUC_VA_LIST: u32 = 1;
pub const __BIONIC__: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __bos_level: u32 = 0;
pub const __ANDROID_API_FUTURE__: u32 = 10000;
pub const __ANDROID_API__: u32 = 10000;
pub const __ANDROID_API_G__: u32 = 9;
pub const __ANDROID_API_I__: u32 = 14;
pub const __ANDROID_API_J__: u32 = 16;
pub const __ANDROID_API_J_MR1__: u32 = 17;
pub const __ANDROID_API_J_MR2__: u32 = 18;
pub const __ANDROID_API_K__: u32 = 19;
pub const __ANDROID_API_L__: u32 = 21;
pub const __ANDROID_API_L_MR1__: u32 = 22;
pub const __ANDROID_API_M__: u32 = 23;
pub const __ANDROID_API_N__: u32 = 24;
pub const __ANDROID_API_N_MR1__: u32 = 25;
pub const __ANDROID_API_O__: u32 = 26;
pub const __ANDROID_API_O_MR1__: u32 = 27;
pub const __ANDROID_API_P__: u32 = 28;
pub const __ANDROID_API_Q__: u32 = 29;
pub const INT8_MIN: i32 = -128;
pub const INT8_MAX: u32 = 127;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST8_MAX: u32 = 127;
pub const UINT8_MAX: u32 = 255;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_FAST8_MAX: u32 = 255;
pub const INT16_MIN: i32 = -32768;
pub const INT16_MAX: u32 = 32767;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const UINT16_MAX: u32 = 65535;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const INT32_MIN: i32 = -2147483648;
pub const INT32_MAX: u32 = 2147483647;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 4294967295;
pub const WINT_MIN: u32 = 0;
pub const JNI_FALSE: u32 = 0;
pub const JNI_TRUE: u32 = 1;
pub const JNI_VERSION_1_1: u32 = 65537;
pub const JNI_VERSION_1_2: u32 = 65538;
pub const JNI_VERSION_1_4: u32 = 65540;
pub const JNI_VERSION_1_6: u32 = 65542;
pub const JNI_OK: u32 = 0;
pub const JNI_ERR: i32 = -1;
pub const JNI_EDETACHED: i32 = -2;
pub const JNI_EVERSION: i32 = -3;
pub const JNI_ENOMEM: i32 = -4;
pub const JNI_EEXIST: i32 = -5;
pub const JNI_EINVAL: i32 = -6;
pub const JNI_COMMIT: u32 = 1;
pub const JNI_ABORT: u32 = 2;
pub const SL_ANDROID_JAVA_PROXY_ROUTING: u32 = 1;

/*---------------------------------------------------------------------------*/
/* Android AudioRecorder configuration                                       */
/*---------------------------------------------------------------------------*/

pub const SL_ANDROID_KEY_RECORDING_PRESET: &'static [SLchar; 23usize] = b"androidRecordingPreset\0";
pub const SL_ANDROID_RECORDING_PRESET_NONE: SLuint32 = 0;
pub const SL_ANDROID_RECORDING_PRESET_GENERIC: SLuint32 = 1;
pub const SL_ANDROID_RECORDING_PRESET_CAMCORDER: SLuint32 = 2;
pub const SL_ANDROID_RECORDING_PRESET_VOICE_RECOGNITION: SLuint32 = 3;
pub const SL_ANDROID_RECORDING_PRESET_VOICE_COMMUNICATION: SLuint32 = 4;
pub const SL_ANDROID_RECORDING_PRESET_UNPROCESSED: SLuint32 = 5;

/*---------------------------------------------------------------------------*/
/* Android AudioPlayer configuration                                         */
/*---------------------------------------------------------------------------*/

pub const SL_ANDROID_KEY_STREAM_TYPE: &'static [SLchar; 26usize] = b"androidPlaybackStreamType\0";
pub const SL_ANDROID_STREAM_VOICE: SLint32 = 0;
pub const SL_ANDROID_STREAM_SYSTEM: SLint32 = 1;
pub const SL_ANDROID_STREAM_RING: SLint32 = 2;
pub const SL_ANDROID_STREAM_MEDIA: SLint32 = 3;
pub const SL_ANDROID_STREAM_ALARM: SLint32 = 4;
pub const SL_ANDROID_STREAM_NOTIFICATION: SLint32 = 5;

/*---------------------------------------------------------------------------*/
/* Android AudioPlayer and AudioRecorder configuration                       */
/*---------------------------------------------------------------------------*/

pub const SL_ANDROID_KEY_PERFORMANCE_MODE: &'static [SLchar; 23usize] = b"androidPerformanceMode\0";
pub const SL_ANDROID_PERFORMANCE_NONE: SLuint32 = 0;
pub const SL_ANDROID_PERFORMANCE_LATENCY: SLuint32 = 1;
pub const SL_ANDROID_PERFORMANCE_LATENCY_EFFECTS: SLuint32 = 2;
pub const SL_ANDROID_PERFORMANCE_POWER_SAVING: SLuint32 = 3;

/*---------------------------------------------------------------------------*/
/* Android File Descriptor Data Locator                                      */
/*---------------------------------------------------------------------------*/

pub const SL_DATALOCATOR_ANDROIDFD: SLuint32 = 0x800007BC;
pub const SL_DATALOCATOR_ANDROIDFD_USE_FILE_SIZE: SLAint64 = -1i64;

/*---------------------------------------------------------------------------*/
/* Android Android Simple Buffer Queue Data Locator                          */
/*---------------------------------------------------------------------------*/

/// Addendum to Data locator macros
pub const SL_DATALOCATOR_ANDROIDSIMPLEBUFFERQUEUE: SLuint32 = 0x800007BD;

/*---------------------------------------------------------------------------*/
/* Android Buffer Queue Data Locator                                         */
/*---------------------------------------------------------------------------*/

/// Addendum to Data locator macros
pub const SL_DATALOCATOR_ANDROIDBUFFERQUEUE: SLuint32 = 0x800007BE;

/// MIME types required for data in Android Buffer Queues
pub const SL_ANDROID_MIME_AACADTS: &'static [SLchar; 27usize] = b"audio/vnd.android.aac-adts\0";

pub const SL_BOOLEAN_FALSE: SLboolean = 0;
pub const SL_BOOLEAN_TRUE: SLboolean = 1;

pub const SL_MILLIBEL_MAX: SLmillibel = 0x7FFF;
pub const SL_MILLIBEL_MIN: SLmillibel = -SL_MILLIBEL_MAX - 1;

pub const SL_MILLIHERTZ_MAX: SLmilliHertz = 0xFFFFFFFF;
pub const SL_MILLIMETER_MAX: SLmillimeter = 0x7FFFFFFF;

/* Objects ID's */

pub const SL_OBJECTID_ENGINE: SLuint32 = 4097;
pub const SL_OBJECTID_LEDDEVICE: SLuint32 = 4098;
pub const SL_OBJECTID_VIBRADEVICE: SLuint32 = 4099;
pub const SL_OBJECTID_AUDIOPLAYER: SLuint32 = 4100;
pub const SL_OBJECTID_AUDIORECORDER: SLuint32 = 4101;
pub const SL_OBJECTID_MIDIPLAYER: SLuint32 = 4102;
pub const SL_OBJECTID_LISTENER: SLuint32 = 4103;
pub const SL_OBJECTID_3DGROUP: SLuint32 = 4104;
pub const SL_OBJECTID_OUTPUTMIX: SLuint32 = 4105;
pub const SL_OBJECTID_METADATAEXTRACTOR: SLuint32 = 4106;

/* SL Profiles */

pub const SL_PROFILES_PHONE: SLuint16 = 1;
pub const SL_PROFILES_MUSIC: SLuint16 = 2;
pub const SL_PROFILES_GAME: SLuint16 = 4;

/* Types of voices supported by the system */

pub const SL_VOICETYPE_2D_AUDIO: SLuint16 = 1;
pub const SL_VOICETYPE_MIDI: SLuint16 = 2;
pub const SL_VOICETYPE_3D_AUDIO: SLuint16 = 4;
pub const SL_VOICETYPE_3D_MIDIOUTPUT: SLuint16 = 8;

/* Convenient macros representing various different priority levels, for use with the SetPriority method */

pub const SL_PRIORITY_LOWEST: SLint32 = -2147483648;
pub const SL_PRIORITY_VERYLOW: SLint32 = -1610612736;
pub const SL_PRIORITY_LOW: SLint32 = -1073741824;
pub const SL_PRIORITY_BELOWNORMAL: SLint32 = -536870912;
pub const SL_PRIORITY_NORMAL: SLint32 = 0;
pub const SL_PRIORITY_ABOVENORMAL: SLint32 = 536870912;
pub const SL_PRIORITY_HIGH: SLint32 = 1073741824;
pub const SL_PRIORITY_VERYHIGH: SLint32 = 1610612736;
pub const SL_PRIORITY_HIGHEST: SLint32 = 2147483647;

/** These macros list the various sample formats that are possible on audio input and output devices. */

pub const SL_PCMSAMPLEFORMAT_FIXED_8: SLuint16 = 8;
pub const SL_PCMSAMPLEFORMAT_FIXED_16: SLuint16 = 16;
pub const SL_PCMSAMPLEFORMAT_FIXED_20: SLuint16 = 20;
pub const SL_PCMSAMPLEFORMAT_FIXED_24: SLuint16 = 24;
pub const SL_PCMSAMPLEFORMAT_FIXED_28: SLuint16 = 28;
pub const SL_PCMSAMPLEFORMAT_FIXED_32: SLuint16 = 32;

/** These macros specify the commonly used sampling rates (in milliHertz) supported by most audio I/O devices. */

pub const SL_SAMPLINGRATE_8: SLuint32 = 8000000;
pub const SL_SAMPLINGRATE_11_025: SLuint32 = 11025000;
pub const SL_SAMPLINGRATE_12: SLuint32 = 12000000;
pub const SL_SAMPLINGRATE_16: SLuint32 = 16000000;
pub const SL_SAMPLINGRATE_22_05: SLuint32 = 22050000;
pub const SL_SAMPLINGRATE_24: SLuint32 = 24000000;
pub const SL_SAMPLINGRATE_32: SLuint32 = 32000000;
pub const SL_SAMPLINGRATE_44_1: SLuint32 = 44100000;
pub const SL_SAMPLINGRATE_48: SLuint32 = 48000000;
pub const SL_SAMPLINGRATE_64: SLuint32 = 64000000;
pub const SL_SAMPLINGRATE_88_2: SLuint32 = 88200000;
pub const SL_SAMPLINGRATE_96: SLuint32 = 96000000;
pub const SL_SAMPLINGRATE_192: SLuint32 = 192000000;

pub const SL_SPEAKER_FRONT_LEFT: SLuint32 = 1;
pub const SL_SPEAKER_FRONT_RIGHT: SLuint32 = 2;
pub const SL_SPEAKER_FRONT_CENTER: SLuint32 = 4;
pub const SL_SPEAKER_LOW_FREQUENCY: SLuint32 = 8;
pub const SL_SPEAKER_BACK_LEFT: SLuint32 = 16;
pub const SL_SPEAKER_BACK_RIGHT: SLuint32 = 32;
pub const SL_SPEAKER_FRONT_LEFT_OF_CENTER: SLuint32 = 64;
pub const SL_SPEAKER_FRONT_RIGHT_OF_CENTER: SLuint32 = 128;
pub const SL_SPEAKER_BACK_CENTER: SLuint32 = 256;
pub const SL_SPEAKER_SIDE_LEFT: SLuint32 = 512;
pub const SL_SPEAKER_SIDE_RIGHT: SLuint32 = 1024;
pub const SL_SPEAKER_TOP_CENTER: SLuint32 = 2048;
pub const SL_SPEAKER_TOP_FRONT_LEFT: SLuint32 = 4096;
pub const SL_SPEAKER_TOP_FRONT_CENTER: SLuint32 = 8192;
pub const SL_SPEAKER_TOP_FRONT_RIGHT: SLuint32 = 16384;
pub const SL_SPEAKER_TOP_BACK_LEFT: SLuint32 = 32768;
pub const SL_SPEAKER_TOP_BACK_CENTER: SLuint32 = 65536;
pub const SL_SPEAKER_TOP_BACK_RIGHT: SLuint32 = 131072;

/*****************************************************************************/
/* Errors                                                                    */
/*                                                                           */
/*****************************************************************************/

pub const SL_RESULT_SUCCESS: SLuint32 = 0;
pub const SL_RESULT_PRECONDITIONS_VIOLATED: SLuint32 = 1;
pub const SL_RESULT_PARAMETER_INVALID: SLuint32 = 2;
pub const SL_RESULT_MEMORY_FAILURE: SLuint32 = 3;
pub const SL_RESULT_RESOURCE_ERROR: SLuint32 = 4;
pub const SL_RESULT_RESOURCE_LOST: SLuint32 = 5;
pub const SL_RESULT_IO_ERROR: SLuint32 = 6;
pub const SL_RESULT_BUFFER_INSUFFICIENT: SLuint32 = 7;
pub const SL_RESULT_CONTENT_CORRUPTED: SLuint32 = 8;
pub const SL_RESULT_CONTENT_UNSUPPORTED: SLuint32 = 9;
pub const SL_RESULT_CONTENT_NOT_FOUND: SLuint32 = 10;
pub const SL_RESULT_PERMISSION_DENIED: SLuint32 = 11;
pub const SL_RESULT_FEATURE_UNSUPPORTED: SLuint32 = 12;
pub const SL_RESULT_INTERNAL_ERROR: SLuint32 = 13;
pub const SL_RESULT_UNKNOWN_ERROR: SLuint32 = 14;
pub const SL_RESULT_OPERATION_ABORTED: SLuint32 = 15;
pub const SL_RESULT_CONTROL_LOST: SLuint32 = 16;

/* Object state definitions */

pub const SL_OBJECT_STATE_UNREALIZED: SLuint32 = 1;
pub const SL_OBJECT_STATE_REALIZED: SLuint32 = 2;
pub const SL_OBJECT_STATE_SUSPENDED: SLuint32 = 3;

/* Object event definitions */
pub const SL_OBJECT_EVENT_RUNTIME_ERROR: SLuint32 = 1;
pub const SL_OBJECT_EVENT_ASYNC_TERMINATION: SLuint32 = 2;
pub const SL_OBJECT_EVENT_RESOURCES_LOST: SLuint32 = 3;
pub const SL_OBJECT_EVENT_RESOURCES_AVAILABLE: SLuint32 = 4;
pub const SL_OBJECT_EVENT_ITF_CONTROL_TAKEN: SLuint32 = 5;
pub const SL_OBJECT_EVENT_ITF_CONTROL_RETURNED: SLuint32 = 6;
pub const SL_OBJECT_EVENT_ITF_PARAMETERS_CHANGED: SLuint32 = 7;

/** Data locator macros  */
pub const SL_DATALOCATOR_URI: SLuint32 = 0x00000001;
pub const SL_DATALOCATOR_ADDRESS: SLuint32 = 0x00000002;
pub const SL_DATALOCATOR_IODEVICE: SLuint32 = 0x00000003;
pub const SL_DATALOCATOR_OUTPUTMIX: SLuint32 = 0x00000004;
pub const SL_DATALOCATOR_RESERVED5: SLuint32 = 0x00000005;
pub const SL_DATALOCATOR_BUFFERQUEUE: SLuint32 = 0x00000006;
pub const SL_DATALOCATOR_MIDIBUFFERQUEUE: SLuint32 = 0x00000007;
pub const SL_DATALOCATOR_RESERVED8: SLuint32 = 0x00000008;

/*---------------------------------------------------------------------------*/
/* Playback interface                                                        */
/*---------------------------------------------------------------------------*/

/** Playback states */
pub const SL_PLAYSTATE_STOPPED: SLuint32 = 0x00000001;
pub const SL_PLAYSTATE_PAUSED: SLuint32 = 0x00000002;
pub const SL_PLAYSTATE_PLAYING: SLuint32 = 0x00000003;

/** Play events **/
pub const SL_PLAYEVENT_HEADATEND: SLuint32 = 0x00000001;
pub const SL_PLAYEVENT_HEADATMARKER: SLuint32 = 0x00000002;
pub const SL_PLAYEVENT_HEADATNEWPOS: SLuint32 = 0x00000004;
pub const SL_PLAYEVENT_HEADMOVING: SLuint32 = 0x00000008;
pub const SL_PLAYEVENT_HEADSTALLED: SLuint32 = 0x00000010;

pub const SL_TIME_UNKNOWN: SLuint32 = 0xFFFFFFFF;

/*---------------------------------------------------------------------------*/
/* Data Source and Data Sink Structures                                      */
/*---------------------------------------------------------------------------*/

/** Data format defines */
pub const SL_DATAFORMAT_MIME: SLuint32 = 1;
pub const SL_DATAFORMAT_PCM: SLuint32 = 2;
pub const SL_DATAFORMAT_RESERVED3: SLuint32 = 3;

/* Byte order of a block of 16- or 32-bit data */
pub const SL_BYTEORDER_BIGENDIAN: SLuint32 = 0x00000001;
pub const SL_BYTEORDER_LITTLEENDIAN: SLuint32 = 0x00000002;

/* Container type */
pub const SL_CONTAINERTYPE_UNSPECIFIED: SLuint32 = 0x00000001;
pub const SL_CONTAINERTYPE_RAW: SLuint32 = 0x00000002;
pub const SL_CONTAINERTYPE_ASF: SLuint32 = 0x00000003;
pub const SL_CONTAINERTYPE_AVI: SLuint32 = 0x00000004;
pub const SL_CONTAINERTYPE_BMP: SLuint32 = 0x00000005;
pub const SL_CONTAINERTYPE_JPG: SLuint32 = 0x00000006;
pub const SL_CONTAINERTYPE_JPG2000: SLuint32 = 0x00000007;
pub const SL_CONTAINERTYPE_M4A: SLuint32 = 0x00000008;
pub const SL_CONTAINERTYPE_MP3: SLuint32 = 0x00000009;
pub const SL_CONTAINERTYPE_MP4: SLuint32 = 0x0000000A;
pub const SL_CONTAINERTYPE_MPEG_ES: SLuint32 = 0x0000000B;
pub const SL_CONTAINERTYPE_MPEG_PS: SLuint32 = 0x0000000C;
pub const SL_CONTAINERTYPE_MPEG_TS: SLuint32 = 0x0000000D;
pub const SL_CONTAINERTYPE_QT: SLuint32 = 0x0000000E;
pub const SL_CONTAINERTYPE_WAV: SLuint32 = 0x0000000F;
pub const SL_CONTAINERTYPE_XMF_0: SLuint32 = 0x00000010;
pub const SL_CONTAINERTYPE_XMF_1: SLuint32 = 0x00000011;
pub const SL_CONTAINERTYPE_XMF_2: SLuint32 = 0x00000012;
pub const SL_CONTAINERTYPE_XMF_3: SLuint32 = 0x00000013;
pub const SL_CONTAINERTYPE_XMF_GENERIC: SLuint32 = 0x00000014;
pub const SL_CONTAINERTYPE_AMR: SLuint32 = 0x00000015;
pub const SL_CONTAINERTYPE_AAC: SLuint32 = 0x00000016;
pub const SL_CONTAINERTYPE_3GPP: SLuint32 = 0x00000017;
pub const SL_CONTAINERTYPE_3GA: SLuint32 = 0x00000018;
pub const SL_CONTAINERTYPE_RM: SLuint32 = 0x00000019;
pub const SL_CONTAINERTYPE_DMF: SLuint32 = 0x0000001A;
pub const SL_CONTAINERTYPE_SMF: SLuint32 = 0x0000001B;
pub const SL_CONTAINERTYPE_MOBILE_DLS: SLuint32 = 0x0000001C;
pub const SL_CONTAINERTYPE_OGG: SLuint32 = 0x0000001D;
