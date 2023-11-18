# An Example in C


Let's consider the following "minimum wrong example" program in C:

```c,editable
#include <stdio.h>
#include <stdlib.h>
#include <sys/stat.h>

int main(int argc, char* argv[]) {
	char *buf, *filename;
	FILE *fp;
	size_t bytes, len;
	struct stat st;

	switch (argc) {
		case 1:
			printf("Too few arguments!\n");
			return 1;

		case 2:
			filename = argv[argc];
			stat(filename, &st);
			len = st.st_size;
			
			buf = (char*)malloc(len);
			if (!buf)
				printf("malloc failed!\n", len);
				return 1;

			fp = fopen(filename, "rb");
			bytes = fread(buf, 1, len, fp);
			if (bytes = st.st_size)
				printf("%s", buf);
			else
				printf("fread failed!\n");

		case 3:
			printf("Too many arguments!\n");
			return 1;
	}

	return 0;
}
```

How many bugs do you spot?

<details>

Despite just 29 lines of code, this C example contains serious bugs in at least 11:

1. Assignment `=` instead of equality comparison `==` (line 28)
2. Excess argument to `printf` (line 23)
3. File descriptor leak (after line 26)
4. Forgotten braces in multi-line `if` (line 22)
5. Forgotten `break` in a `switch` statement (line 32)
6. Forgotten NUL-termination of the `buf` string, leading to a buffer overflow (line 29)
7. Memory leak by not freeing the `malloc`-allocated buffer (line 21)
8. Out-of-bounds access (line 17)
9. Unchecked cases in the `switch` statement (line 11)
10. Unchecked return values of `stat` and `fopen` (lines 18 and 26)

_Shouldn't these bugs be obvious even for a C compiler?_  
No, surprisingly this code compiles warning-free at the default warning level, even in the latest GCC version (13.2 as of writing).

_Isn't this a highly unrealistic example?_  
Absolutely not, these kind of bugs have lead to serious security vulnerabilities in the past. Some examples:

* Assignment `=` instead of equality comparison `==`: [The Linux Backdoor Attempt of 2003](https://freedom-to-tinker.com/2013/10/09/the-linux-backdoor-attempt-of-2003)
* Forgotten braces in multi-line `if`: [The Apple goto fail vulnerability](https://dwheeler.com/essays/apple-goto-fail.html)
* Forgotten `break` in a `switch` statement: [The break that broke sudo](https://nakedsecurity.sophos.com/2012/05/21/anatomy-of-a-security-hole-the-break-that-broke-sudo)

_How is Rust any better here?_  
Safe Rust makes all of these bugs impossible:

1. Assignments inside an `if` clause are not supported.
2. Format strings are checked at compile-time.
3. Resources are freed at the end of scope via the `Drop` trait.
4. All `if` clauses require braces.
5. `match` (as the Rust equivalent to `switch`) does not fall-through, hence you can't accidentally forget a `break`.
6. Buffer slices carry their size and don't rely on a NUL terminator.
7. Heap-allocated memory is freed via the `Drop` trait when the corresponding `Box` leaves the scope.
8. Out-of-bounds accesses cause a panic or can be checked via the `get` method of a slice.
9. `match` mandates that all cases are handled.
10. Fallible Rust functions return `Result` values that need to be unwrapped and thereby checked for success.
    Additionally, the compiler emits a warning if you miss to check the return value of a function marked with `#[must_use]`.

</details>
