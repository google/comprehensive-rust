#include "include/blobstore.h"
#include "main.rs.h"
#include <algorithm>
#include <functional>

namespace org {
namespace blobstore {

BlobstoreClient::BlobstoreClient() {}

// Upload a new blob and return a blobid that serves as a handle to the blob.
uint64_t BlobstoreClient::put(MultiBuf &buf) {
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
  blobs[blobid] = {std::move(contents), {}};
  return blobid;
}

// Add tag to an existing blob.
void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) {
  blobs[blobid].tags.emplace(tag);
}

// Retrieve metadata about a blob.
BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const {
  BlobMetadata metadata{};
  auto blob = blobs.find(blobid);
  if (blob != blobs.end()) {
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
