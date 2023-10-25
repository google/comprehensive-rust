#include "demo/include/blobstore.h"
#include "demo/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>

namespace org {
namespace blobstore {

// Toy implementation of an in-memory blobstore.
//
// In reality the implementation of BlobstoreClient could be a large complex C++
// library.
class BlobstoreClient::impl {
  friend BlobstoreClient;
  using Blob = struct {
    std::string data;
    std::set<std::string> tags;
  };
  std::unordered_map<uint64_t, Blob> blobs;
};

BlobstoreClient::BlobstoreClient() : impl(new class BlobstoreClient::impl) {}

// Upload a new blob and return a blobid that serves as a handle to the blob.
uint64_t BlobstoreClient::put(MultiBuf &buf) const {
  std::string contents;

  // Traverse the caller's chunk iterator.
  //
  // In reality there might be sophisticated batching of chunks and/or parallel
  // upload implemented by the blobstore's C++ client.
  while (true) {
    auto chunk = next_chunk(buf);
    if (chunk.size() == 0) {
      break;
    }
    contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
  }

  // Insert into map and provide caller the handle.
  auto blobid = std::hash<std::string>{}(contents);
  impl->blobs[blobid] = {std::move(contents), {}};
  return blobid;
}

// Add tag to an existing blob.
void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) const {
  impl->blobs[blobid].tags.emplace(tag);
}

// Retrieve metadata about a blob.
BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const {
  BlobMetadata metadata{};
  auto blob = impl->blobs.find(blobid);
  if (blob != impl->blobs.end()) {
    metadata.size = blob->second.data.size();
    std::for_each(blob->second.tags.cbegin(), blob->second.tags.cend(),
                  [&](auto &t) { metadata.tags.emplace_back(t); });
  }
  return metadata;
}

std::unique_ptr<BlobstoreClient> new_blobstore_client() {
  return std::make_unique<BlobstoreClient>();
}

} // namespace blobstore
} // namespace org
