use std::sync::Arc;
use parking_lot::Mutex;

pub struct AudioRenderer {
    internal: Arc<Mutex<RendererInternal>>,

    #[allow(dead_code)]
    audio_track: RtcAudioTrack,
}

struct RendererInternal {
    sample_rate: u32,
    channels: u8,
    audio_data: Vec<i16>,
}

impl AudioRenderer {
    pub fn new(
        async_handle: &tokio::runtime::Handle,
        audio_track: RtcAudioTrack,
    ) -> Self {
        let internal = Arc::new(Mutex::new(RendererInternal {
            sample_rate: 0,
            channels: 0,
            audio_data: Vec::default(),
        }));

        // TODO: Gracefully close the thread
        let mut audio_sink = NativeAudioStream::new(audio_track.clone());

        std::thread::spawn({
            let async_handle = async_handle.clone();
            let internal = internal.clone();
            move || {
            }
        });

        AudioRenderer {
            internal,
            audio_track,
        }
    }
}