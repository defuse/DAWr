
pub const SAMPLE_RATE : u32 = 44100;
pub const CHUNK_SIZE : usize = 4096;
/* This won't create a bug unless people are working with eons of audio. */
pub const TIME_INFINITY : u64 = 0xFFFFFFFFFFFFFFFF;
