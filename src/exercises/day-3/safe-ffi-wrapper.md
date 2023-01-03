# Safe FFI Wrapper

러스트는 _외부 기능 호출(FFI)_을 지원합니다. 
이를 이용하여 C에서 사용할 폴더 내 파일이름을 읽어오는 `glibc`함수를 만들 것입니다.

아래 리눅스 메뉴얼 문서들을 참조하시기 바랍니다: 
* [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
* [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
* [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)

아래 코드를 <https://play.rust-lang.org/>에 복사하고 빠진 함수와 메서드를 채워봅니다.

> Rust has great support for calling functions through a _foreign function
> interface_ (FFI). We will use this to build a safe wrapper the `glibc` functions
> you would use from C to read the filenames of a directory.
> 
> You will want to consult the manual pages:
> 
> * [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
> * [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
> * [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)
> 
> You will also want to browse the [`std::ffi`] module, particular for [`CStr`]
> and [`CString`] types which are used to hold NUL-terminated strings coming from
> C. The [Nomicon] also has a very useful chapter about FFI.
> 
> [`std::ffi`]: https://doc.rust-lang.org/std/ffi/
> [`CStr`]: https://doc.rust-lang.org/std/ffi/struct.CStr.html
> [`CString`]: https://doc.rust-lang.org/std/ffi/struct.CString.html
> [Nomicon]: https://doc.rust-lang.org/nomicon/ffi.html
> 
> Copy the code below to <https://play.rust-lang.org/> and fill in the missing
> functions and methods:

```rust,should_panic
// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

{{#include safe-ffi-wrapper.rs:ffi}}

{{#include safe-ffi-wrapper.rs:DirectoryIterator}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:Iterator}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:Drop}}
        unimplemented!()
    }
}

{{#include safe-ffi-wrapper.rs:main}}
```

---
역주
- DirectoryIterator, Iterator, Drop impl 블록을 채우면 됩니다.
- 참조문서를 싹 훝어도 어려운데(...)
