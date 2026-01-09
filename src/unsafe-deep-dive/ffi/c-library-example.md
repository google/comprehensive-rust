# C Library Example

```c
#ifndef TEXT_ANALYSIS_H
#define TEXT_ANALYSIS_H

#include <stddef.h>
#include <stdbool.h>

typedef struct TextAnalyst TextAnalyst;

typedef struct {
    const char* start;
    size_t length;
    size_t index;
} Token;

typedef enum {
    TA_OK = 0,
    TA_ERR_NULL_POINTER,
    TA_ERR_OUT_OF_MEMORY,
    TA_ERR_OTHER,
} TAError;

/* Return `false` to indicate that no token was found. */ 
typedef bool (*Tokenizer)(Token* token, void* extra_context);


typedef bool (*TokenCallback)(void* user_context, Token* token, void* result);

/* TextAnalyst constructor */
TextAnalyst* ta_new(void);

/* TextAnalyst destructor */
void ta_free(TextAnalyst* ta);

/* Resets state to clear the current document */ 
void ta_reset(TextAnalyst* ta);

/* Use custom tokenizer (defaults to whitespace) */ 
void ta_set_tokenizer(TextAnalyst* ta, Tokenizer* func);

TAError ta_set_text(TextAnalyst* ta, const char* text, size_t len, bool make_copy);

/* Apply `callback` to each token */
size_t ta_foreach_token(const TextAnalyst* ta, const TokenCallback* callback, void* user_context);

/* Get human-readable error message */
const char* ta_error_string(TAError error);

#endif /* TEXT_ANALYSIS_H */
```

<details>

C libraries will hide their implementation details with a `void*` argument.

Consider this header file of a natural language processing library that hides
the `TextAnalyst` and `Analysis` types.

This can be emulated in Rust with a type similar to this:

```rust
#[repr(C)]
pub struct TextAnalyst {
    _private: [u8; 0],
}
```

Exercise: Ask learners to wrap this library.

_Suggested Solution_

```rust
// ffi.rs
use std::ffi::c_char;
use std::os::raw::c_void;

#[repr(C)]
pub struct TextAnalyst {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Token {
    pub start: *const c_char,
    pub length: usize,
    pub index: usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TAError {
    Ok = 0,
    NullPointer = 1,
    OutOfMemory = 2,
    Other = 3,
}

pub type Tokenizer = Option<
    unsafe extern "C" fn(token: *mut Token, extra_context: *mut c_void) -> bool,
>;

pub type TokenCallback = Option<
    unsafe extern "C" fn(
        user_context: *mut c_void,
        token: *mut Token,
        result: *mut c_void,
    ) -> bool,
>;

unsafe extern "C" {
    pub fn ta_new() -> *mut TextAnalyst;

    pub fn ta_free(ta: *mut TextAnalyst);

    pub fn ta_reset(ta: *mut TextAnalyst);

    pub fn ta_set_tokenizer(ta: *mut TextAnalyst, func: *const Tokenizer);

    pub fn ta_set_text(
        ta: *mut TextAnalyst,
        text: *const c_char,
        len: usize,
        make_copy: bool,
    ) -> TAError;

    pub fn ta_foreach_token(
        ta: *const TextAnalyst,
        callback: *const TokenCallback,
        user_context: *mut c_void,
    ) -> usize;

    pub fn ta_error_string(error: TAError) -> *const c_char;
}
```

</details>
