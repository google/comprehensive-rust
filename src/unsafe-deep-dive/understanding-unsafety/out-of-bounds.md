# Out of bounds access

The way that we often think of memory as application programmers, as a linear
block of a space that we can reserve space from and give back to, is somewhat of
an illusion.

---

## A motivating example

```cpp
int numbers[10] = {};

bool numbers_contains(int n)
{
    for (int i = 0; i <= 10; i++) {
        if (table[i] == v) return true;
    }
    return false;
}
```

> Derived from the Undefined Behavior chapter from cppreference.com
> <https://en.cppreference.com/w/cpp/language/ub.html>

<details>

The `numbers` array contains no members, and therefore should be false for all
inputs. However, gcc13 with -O2 optimizes this code to ensure that it returns
true for all cases.

```asm
numbers_contains(int):
        mov     eax, 1
        ret
numbers:
        .zero   16
```

</details>

---

Hello there

<details>

More details

</details>
