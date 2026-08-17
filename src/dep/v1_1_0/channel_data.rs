use std::sync::ARc;
use memmap2::Mmap;

// Iterator for reading pvp and corresponding signal vector
pub struct ChannelDataIterator  {

    mmap: Arc<Mmap>,
    channel_info: ChannelInfo,
    pvp_parameters: Vec<PvpParameter>,
    current_vector: usize,
    total_vectors: usize,
}
