#pragma once
#include "rust/cxx.h"
#include <memory>
#include <set>
#include <unordered_map>
#include <string>

namespace org {
namespace blobstore {

struct MultiBuf;
struct BlobMetadata;

class BlobstoreClient {
public:
  BlobstoreClient();
  uint64_t put(MultiBuf &buf);
  void tag(uint64_t blobid, rust::Str tag);
  BlobMetadata metadata(uint64_t blobid) const;

private:
  using Blob = struct {
    std::string data;
    std::set<std::string> tags;
  };
  std::unordered_map<uint64_t, Blob> blobs;
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();

} // namespace blobstore
} // namespace org
