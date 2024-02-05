//! Example project demonstrating usage of CXX.
// ANCHOR: bridge
#[allow(unsafe_op_in_unsafe_fn)]
#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Shared structs with fields visible to both languages.
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // ANCHOR: rust_bridge
    // Rust types and signatures exposed to C++.
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }
    // ANCHOR_END: rust_bridge

    // ANCHOR: cpp_bridge
    // C++ types and signatures exposed to Rust.
    unsafe extern "C++" {
        include!("include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(self: Pin<&mut BlobstoreClient>, parts: &mut MultiBuf) -> u64;
        fn tag(self: Pin<&mut BlobstoreClient>, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
    // ANCHOR_END: cpp_bridge
}
// ANCHOR_END: bridge

/// An iterator over contiguous chunks of a discontiguous file object.
///
/// Toy implementation uses a Vec<Vec<u8>> but in reality this might be
/// iterating over some more complex Rust data structure like a rope, or maybe
/// loading chunks lazily from somewhere.
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

/// Pulls the next chunk from the buffer.
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let mut client = ffi::new_blobstore_client();

    // Upload a blob.
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.pin_mut().put(&mut buf);
    println!("blobid = {}", blobid);

    // Add a tag.
    client.pin_mut().tag(blobid, "rust");

    // Read back the tags.
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
